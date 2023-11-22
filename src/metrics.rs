// Copyright 2023 TiKV Project Authors. Licensed under Apache-2.0.

//! Metrics of the grpc pool.

use lazy_static::lazy_static;
use prometheus::*;

lazy_static! {
    // Grpc pool io handle duration .
    pub static ref GRPC_POOL_IO_HANDLE_DURATION: HistogramVec = register_histogram_vec!(
        "grpc_pool_io_handle_duration",
        "Bucketed histogram of grpc pool wait duration from the completion queue",
        &["name"],
        exponential_buckets(1e-7, 2.0, 20).unwrap() // 100ns ~ 100ms
    )
    .unwrap();

    // Grpc handle execute duration
    pub static ref GRPC_POOL_EXECUTE_DURATION: HistogramVec = register_histogram_vec!(
        "grpc_pool_execute_duration",
        "Bucketed histogram of grpc pool execute duration for every time",
        &["name"],
        exponential_buckets(1e-7, 2.0, 20).unwrap() // 100ns ~ 100ms
    )
    .unwrap();

    // Grpc pool event count task .
    pub static ref GRPC_POOL_EVENT_COUNT_VEC: IntCounterVec = register_int_counter_vec!(
        "grpc_pool_event_task_count",
        "Total event task count in grpc pool",
        &["name","event"]
    )
    .unwrap();
}
