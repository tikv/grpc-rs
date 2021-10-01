// Copyright 2021 TiKV Project Authors. Licensed under Apache-2.0.

//! Channelz provides channel level debug information. In short, There are four types of
//! top level entities: channel, subchannel, socket and server. All entities are
//! identified by an positive unique integer, which is allocated in order. For more
//! explanation, see https://github.com/grpc/proposal/blob/master/A14-channelz.md.
//!
//! A full support requires a service that allow remote querying. But for now it's
//! too complicated to add full support. Because gRPC C core exposes the information
//! using JSON format, and there is no protobuf library that supports parsing json
//! format in Rust. So this module only provides safe APIs to access the informations.

use std::ffi::CStr;
use std::{cmp, str};

macro_rules! visit {
    ($ptr:expr, $visitor:ident) => {{
        let s_ptr = $ptr;
        let res;
        if !s_ptr.is_null() {
            let c_s = CStr::from_ptr(s_ptr);
            // It's json string, so it must be utf8 compatible.
            let s = str::from_utf8_unchecked(c_s.to_bytes());
            res = $visitor(s);
            grpcio_sys::gpr_free(s_ptr as _);
        } else {
            res = $visitor("");
        }
        res
    }};
}

/// Gets all root channels (i.e. channels the application has directly created). This
/// does not include subchannels nor non-top level channels.
pub fn get_top_channels<V, R>(start_channel_id: u64, visitor: V) -> R
where
    V: FnOnce(&str) -> R,
{
    unsafe {
        visit!(
            grpcio_sys::grpc_channelz_get_top_channels(start_channel_id as _),
            visitor
        )
    }
}

/// Gets all servers that exist in the process.
pub fn get_servers<V, R>(start_server_id: u64, visitor: V) -> R
where
    V: FnOnce(&str) -> R,
{
    unsafe {
        visit!(
            grpcio_sys::grpc_channelz_get_servers(start_server_id as _),
            visitor
        )
    }
}

/// Returns a single Server, or else an empty string.
pub fn get_server<V, R>(server_id: u64, visitor: V) -> R
where
    V: FnOnce(&str) -> R,
{
    unsafe {
        visit!(
            grpcio_sys::grpc_channelz_get_server(server_id as _),
            visitor
        )
    }
}

/// Gets all server sockets that exist in the server.
pub fn get_server_sockets<V, R>(
    server_id: u64,
    start_socket_id: u64,
    max_results: usize,
    visitor: V,
) -> R
where
    V: FnOnce(&str) -> R,
{
    let max_results = cmp::min(isize::MAX as usize, max_results) as isize;
    unsafe {
        visit!(
            grpcio_sys::grpc_channelz_get_server_sockets(
                server_id as _,
                start_socket_id as _,
                max_results
            ),
            visitor
        )
    }
}

/// Returns a single Channel, or else an empty string.
pub fn get_channel<V, R>(channel_id: u64, visitor: V) -> R
where
    V: FnOnce(&str) -> R,
{
    unsafe {
        visit!(
            grpcio_sys::grpc_channelz_get_channel(channel_id as _),
            visitor
        )
    }
}

/// Returns a single Subchannel, or else an empty string.
pub fn get_subchannel<V, R>(subchannel_id: u64, visitor: V) -> R
where
    V: FnOnce(&str) -> R,
{
    unsafe {
        visit!(
            grpcio_sys::grpc_channelz_get_subchannel(subchannel_id as _),
            visitor
        )
    }
}

/// Returns a single Socket, or else an empty string.
pub fn get_socket<V, R>(socket_id: u64, visitor: V) -> R
where
    V: FnOnce(&str) -> R,
{
    unsafe {
        visit!(
            grpcio_sys::grpc_channelz_get_socket(socket_id as _),
            visitor
        )
    }
}
