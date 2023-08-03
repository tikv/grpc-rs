// Copyright 2021 TiKV Project Authors. Licensed under Apache-2.0.

use crate::proto::{Health, HealthCheckRequest, HealthCheckResponse};
use futures_util::{FutureExt as _, SinkExt as _, Stream, StreamExt as _};
use grpcio::{RpcContext, RpcStatus, RpcStatusCode, ServerStreamingSink, UnarySink, WriteFlags};
use log::info;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

#[cfg(feature = "protobuf-codec")]
use protobuf::ProtobufEnum;

#[cfg(any(feature = "prost-codec", feature = "protobuf-codec"))]
use crate::proto::ServingStatus;

#[cfg(feature = "protobufv3-codec")]
use crate::proto::health_check_response::ServingStatus;

const VERSION_STEP: usize = 8;
const STATUS_MASK: usize = 7;

#[cfg(any(feature = "prost-codec", feature = "protobuf-codec"))]
fn state_to_status(state: usize) -> ServingStatus {
    ServingStatus::from_i32((state & STATUS_MASK) as i32).unwrap()
}

#[cfg(feature = "protobufv3-codec")]
fn state_to_status(state: usize) -> ServingStatus {
    ::protobufv3::Enum::from_i32((state & STATUS_MASK) as i32).unwrap()
}

/// Struct that stores the state of a service and wake all subscribers when there
/// is any updates.
struct StatusCast {
    state: AtomicUsize,
    subscribers: Mutex<HashMap<u64, Waker>>,
}

impl StatusCast {
    fn new(status: ServingStatus) -> StatusCast {
        StatusCast {
            state: AtomicUsize::new(VERSION_STEP | (status as usize)),
            subscribers: Mutex::default(),
        }
    }

    /// Updates the status to specified one and update version.
    fn broadcast(&self, status: ServingStatus) {
        let mut subscribers = self.subscribers.lock().unwrap();
        let state = self.state.load(Ordering::Relaxed);
        let new_state = ((state + VERSION_STEP) & !STATUS_MASK) | (status as usize);
        self.state.store(new_state, Ordering::Relaxed);

        for (_, s) in subscribers.drain() {
            s.wake();
        }
    }
}

/// Struct that gets notified when service status changes.
struct StatusSubscriber {
    cast: Arc<StatusCast>,
    last_state: usize,
    id: u64,
}

impl StatusSubscriber {
    fn new(id: u64, cast: Arc<StatusCast>) -> StatusSubscriber {
        StatusSubscriber {
            cast,
            last_state: 0,
            id,
        }
    }
}

impl Stream for StatusSubscriber {
    type Item = ServingStatus;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<ServingStatus>> {
        let s = &mut *self;
        let cur_state = s.cast.state.load(Ordering::Relaxed);
        if cur_state != s.last_state {
            let status = state_to_status(cur_state);
            s.last_state = cur_state;
            return Poll::Ready(Some(status));
        }

        let mut subscribers = s.cast.subscribers.lock().unwrap();

        let cur_state = s.cast.state.load(Ordering::Relaxed);
        if cur_state != s.last_state {
            let status = state_to_status(cur_state);
            s.last_state = cur_state;
            return Poll::Ready(Some(status));
        }

        match subscribers.entry(s.id) {
            Entry::Occupied(mut e) => {
                if !e.get().will_wake(cx.waker()) {
                    e.insert(cx.waker().clone());
                }
            }
            Entry::Vacant(v) => {
                v.insert(cx.waker().clone());
            }
        }
        Poll::Pending
    }
}

impl Drop for StatusSubscriber {
    fn drop(&mut self) {
        let mut subscribers = self.cast.subscribers.lock().unwrap();
        subscribers.remove(&self.id);
    }
}

#[derive(Default)]
struct Inner {
    id: u64,
    shutdown: bool,
    status: HashMap<String, ServingStatus>,
    casts: HashMap<String, Arc<StatusCast>>,
}

/// Simple implementation for `Health` service.
#[derive(Clone, Default)]
pub struct HealthService {
    inner: Arc<Mutex<Inner>>,
}

impl HealthService {
    /// Resets the serving status of a service or inserts a new service status.
    pub fn set_serving_status(&self, service: &str, status: ServingStatus) {
        let cast = {
            let mut inner = self.inner.lock().unwrap();
            if inner.shutdown {
                info!("health: status changing for {} to {:?} is ignored because health service is shutdown", service, status);
                return;
            }

            if let Some(val) = inner.status.get_mut(service) {
                *val = status;
            } else {
                inner.status.insert(service.to_string(), status);
            }

            if let Some(cast) = inner.casts.get(service) {
                cast.clone()
            } else {
                return;
            }
        };
        cast.broadcast(status);
    }

    /// Sets all serving status to NotServing, and configures the server to
    /// ignore all future status changes.
    ///
    /// This changes serving status for all services.
    pub fn shutdown(&self) {
        let mut inner = self.inner.lock().unwrap();
        inner.shutdown = true;
        for val in inner.status.values_mut() {
            *val = ServingStatus::NotServing;
        }
        for cast in inner.casts.values() {
            cast.broadcast(ServingStatus::NotServing);
        }
    }
}

#[allow(clippy::useless_conversion)]
fn build_response(status: ServingStatus) -> HealthCheckResponse {
    HealthCheckResponse {
        status: status.into(),
        ..Default::default()
    }
}

impl Health for HealthService {
    fn check(
        &mut self,
        ctx: RpcContext,
        req: HealthCheckRequest,
        sink: UnarySink<HealthCheckResponse>,
    ) {
        let status = {
            let inner = self.inner.lock().unwrap();
            inner.status.get(&req.service).cloned()
        };
        if let Some(status) = status {
            let resp = build_response(status);
            ctx.spawn(sink.success(resp).map(|_| ()));
            return;
        }
        ctx.spawn(
            sink.fail(RpcStatus::with_message(
                RpcStatusCode::NOT_FOUND,
                "unknown service".to_owned(),
            ))
            .map(|_| ()),
        )
    }

    fn watch(
        &mut self,
        ctx: RpcContext,
        req: HealthCheckRequest,
        mut sink: ServerStreamingSink<HealthCheckResponse>,
    ) {
        let name = req.service;
        let (id, v) = {
            let mut inner = self.inner.lock().unwrap();
            inner.id += 1;
            if let Some(c) = inner.casts.get(&name) {
                (inner.id, c.clone())
            } else {
                let status = match inner.status.get(&name) {
                    Some(s) => *s,
                    None => ServingStatus::ServiceUnknown,
                };
                let c = Arc::new(StatusCast::new(status));
                inner.casts.insert(name.clone(), c.clone());
                (inner.id, c)
            }
        };
        let sub = StatusSubscriber::new(id, v);
        let inner = self.inner.clone();
        ctx.spawn(async move {
            let _ = sink
                .send_all(&mut sub.map(|s| Ok((build_response(s), WriteFlags::default()))))
                .await;
            let mut inner = inner.lock().unwrap();
            if let Some(c) = inner.casts.get(&name) {
                // If there is any subscriber, then cast reference count should not be 1 as
                // it's referenced by all subscriber.
                if Arc::strong_count(c) == 1 {
                    inner.casts.remove(&name);
                }
            }
        })
    }
}
