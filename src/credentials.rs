// Copyright 2017 PingCAP, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.


use std::ffi::CString;
use std::ptr;

use grpc_sys::{self, GrpcChannelCredentials, GrpcServerCredentials};
use libc::c_char;

pub struct ServerCredentialsBuilder {
    root: Option<CString>,
    cert_chains: Vec<*mut c_char>,
    private_keys: Vec<*mut c_char>,
    force_client_auth: bool,
}

impl ServerCredentialsBuilder {
    pub fn new() -> ServerCredentialsBuilder {
        ServerCredentialsBuilder {
            root: None,
            cert_chains: vec![],
            private_keys: vec![],
            force_client_auth: false,
        }
    }

    pub fn root_cert<S: Into<Vec<u8>>>(mut self,
                                       cert: S,
                                       force_client_auth: bool)
                                       -> ServerCredentialsBuilder {
        self.root = Some(CString::new(cert).unwrap());
        self.force_client_auth = force_client_auth;
        self
    }

    pub fn add_cert<S: Into<Vec<u8>>>(mut self,
                                      cert: S,
                                      private_key: S)
                                      -> ServerCredentialsBuilder {
        self.cert_chains
            .push(CString::new(cert).unwrap().into_raw());
        self.private_keys
            .push(CString::new(private_key).unwrap().into_raw());
        self
    }

    pub fn build(mut self) -> ServerCredentials {
        let root_cert = self.root
            .take()
            .map_or_else(ptr::null_mut, CString::into_raw);
        let cert_chains = self.cert_chains.as_mut_ptr();
        let private_keys = self.private_keys.as_mut_ptr();
        let force_auth = if self.force_client_auth { 1 } else { 0 };

        let credentials = unsafe {
            grpc_sys::grpcwrap_ssl_server_credentials_create(root_cert,
                                                             cert_chains as _,
                                                             private_keys as _,
                                                             self.cert_chains.len(),
                                                             force_auth)
        };

        if !root_cert.is_null() {
            unsafe {
                CString::from_raw(root_cert);
            }
        }

        ServerCredentials { creds: credentials }
    }
}

impl Drop for ServerCredentialsBuilder {
    fn drop(&mut self) {
        for cert in self.cert_chains.drain(..) {
            unsafe {
                CString::from_raw(cert);
            }
        }
        for key in self.private_keys.drain(..) {
            unsafe {
                CString::from_raw(key);
            }
        }
    }
}

pub struct ServerCredentials {
    creds: *mut GrpcServerCredentials,
}

impl ServerCredentials {
    pub fn as_mut_ptr(&mut self) -> *mut GrpcServerCredentials {
        self.creds
    }
}

impl Drop for ServerCredentials {
    fn drop(&mut self) {
        unsafe { grpc_sys::grpc_server_credentials_release(self.creds) }
    }
}

pub struct ChannelCredentialsBuilder {
    root: Option<CString>,
    cert_key_pair: Option<(CString, CString)>,
}

impl ChannelCredentialsBuilder {
    pub fn new() -> ChannelCredentialsBuilder {
        ChannelCredentialsBuilder {
            root: None,
            cert_key_pair: None,
        }
    }

    pub fn root_cert<S: Into<Vec<u8>>>(mut self, cert: S) -> ChannelCredentialsBuilder {
        self.root = Some(CString::new(cert).unwrap());
        self
    }

    pub fn cert<S: Into<Vec<u8>>>(mut self, cert: S, key: S) -> ChannelCredentialsBuilder {
        self.cert_key_pair = Some((CString::new(cert).unwrap(), CString::new(key).unwrap()));
        self
    }

    pub fn build(mut self) -> ChannelCredentials {
        let root_ptr = self.root
            .take()
            .map_or_else(ptr::null_mut, CString::into_raw);
        let (cert_ptr, key_ptr) = self.cert_key_pair
            .take()
            .map_or_else(|| (ptr::null_mut(), ptr::null_mut()),
                         |(cert, key)| (cert.into_raw(), key.into_raw()));

        let creds =
            unsafe { grpc_sys::grpcwrap_ssl_credentials_create(root_ptr, cert_ptr, key_ptr) };

        for ptr in &[root_ptr, cert_ptr, key_ptr] {
            if !ptr.is_null() {
                unsafe {
                    CString::from_raw(*ptr);
                }
            }
        }

        ChannelCredentials { creds: creds }
    }
}

pub struct ChannelCredentials {
    creds: *mut GrpcChannelCredentials,
}

impl ChannelCredentials {
    pub fn as_mut_ptr(&mut self) -> *mut GrpcChannelCredentials {
        self.creds
    }
}

impl Drop for ChannelCredentials {
    fn drop(&mut self) {
        unsafe { grpc_sys::grpc_channel_credentials_release(self.creds) }
    }
}
