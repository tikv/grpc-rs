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

use error::{Error, Result};
use grpc_sys::{self, GrpcChannelCredentials, GrpcServerCredentials};
use libc::c_char;

fn clear_key_securely(key: &mut [u8]) {
    unsafe {
        for b in key {
            ptr::write_volatile(b, 0)
        }
    }
}

/// [`ServerCredentials`] factory in order to configure the properties.
pub struct ServerCredentialsBuilder {
    root: Option<CString>,
    cert_chains: Vec<*mut c_char>,
    private_keys: Vec<*mut c_char>,
    force_client_auth: bool,
}

impl ServerCredentialsBuilder {
    /// Initialize a new [`ServerCredentialsBuilder`].
    pub fn new() -> ServerCredentialsBuilder {
        ServerCredentialsBuilder {
            root: None,
            cert_chains: vec![],
            private_keys: vec![],
            force_client_auth: false,
        }
    }

    /// Set the PEM encoded client root certificate to verify client's identity. If
    /// `force_client_auth` is set to `true`, the authenticity of client check will be enforced.
    pub fn root_cert<S: Into<Vec<u8>>>(
        mut self,
        cert: S,
        force_client_auth: bool,
    ) -> ServerCredentialsBuilder {
        self.root = Some(CString::new(cert).unwrap());
        self.force_client_auth = force_client_auth;
        self
    }

    /// Add a PEM encoded server side certificate and key.
    pub fn add_cert(mut self, cert: Vec<u8>, mut private_key: Vec<u8>) -> ServerCredentialsBuilder {
        if private_key.capacity() == private_key.len() {
            let mut nil_key = Vec::with_capacity(private_key.len() + 1);
            nil_key.extend_from_slice(&private_key);
            clear_key_securely(&mut private_key);
            private_key = nil_key;
        }
        self.cert_chains
            .push(CString::new(cert).unwrap().into_raw());
        self.private_keys
            .push(CString::new(private_key).unwrap().into_raw());
        self
    }

    /// Finalize the [`ServerCredentialsBuilder`] and build the [`ServerCredentials`].
    pub fn build(mut self) -> ServerCredentials {
        let root_cert = self
            .root
            .take()
            .map_or_else(ptr::null_mut, CString::into_raw);
        let cert_chains = self.cert_chains.as_mut_ptr();
        let private_keys = self.private_keys.as_mut_ptr();
        let force_auth = if self.force_client_auth { 1 } else { 0 };

        let credentials = unsafe {
            grpc_sys::grpcwrap_ssl_server_credentials_create(
                root_cert,
                cert_chains as _,
                private_keys as _,
                self.cert_chains.len(),
                force_auth,
            )
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
            let s = unsafe { CString::from_raw(key) };
            clear_key_securely(&mut s.into_bytes_with_nul());
        }
    }
}

/// Server-side SSL credentials.
///
/// Use [`ServerCredentialsBuilder`] to build a [`ServerCredentials`].
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

/// [`ChannelCredentials`] factory in order to configure the properties.
pub struct ChannelCredentialsBuilder {
    root: Option<CString>,
    cert_key_pair: Option<(CString, CString)>,
}

impl ChannelCredentialsBuilder {
    /// Initialize a new [`ChannelCredentialsBuilder`].
    pub fn new() -> ChannelCredentialsBuilder {
        ChannelCredentialsBuilder {
            root: None,
            cert_key_pair: None,
        }
    }

    /// Set the PEM encoded server root certificate to verify server's identity.
    pub fn root_cert(mut self, cert: Vec<u8>) -> ChannelCredentialsBuilder {
        self.root = Some(CString::new(cert).unwrap());
        self
    }

    /// Set the PEM encoded client side certificate and key.
    pub fn cert(mut self, cert: Vec<u8>, mut private_key: Vec<u8>) -> ChannelCredentialsBuilder {
        if private_key.capacity() == private_key.len() {
            let mut nil_key = Vec::with_capacity(private_key.len() + 1);
            nil_key.extend_from_slice(&private_key);
            clear_key_securely(&mut private_key);
            private_key = nil_key;
        }
        self.cert_key_pair = Some((
            CString::new(cert).unwrap(),
            CString::new(private_key).unwrap(),
        ));
        self
    }

    /// Finalize the [`ChannelCredentialsBuilder`] and build the [`ChannelCredentials`].
    pub fn build(mut self) -> ChannelCredentials {
        let root_ptr = self
            .root
            .take()
            .map_or_else(ptr::null_mut, CString::into_raw);
        let (cert_ptr, key_ptr) = self.cert_key_pair.take().map_or_else(
            || (ptr::null_mut(), ptr::null_mut()),
            |(cert, key)| (cert.into_raw(), key.into_raw()),
        );

        let creds =
            unsafe { grpc_sys::grpcwrap_ssl_credentials_create(root_ptr, cert_ptr, key_ptr) };

        if !root_ptr.is_null() {
            unsafe {
                self.root = Some(CString::from_raw(root_ptr));
            }
        }

        if !cert_ptr.is_null() {
            unsafe {
                let cert = CString::from_raw(cert_ptr);
                let key = CString::from_raw(key_ptr);
                self.cert_key_pair = Some((cert, key));
            }
        }

        ChannelCredentials { creds }
    }
}

impl Drop for ChannelCredentialsBuilder {
    fn drop(&mut self) {
        if let Some((_, key)) = self.cert_key_pair.take() {
            clear_key_securely(&mut key.into_bytes_with_nul());
        }
    }
}

/// Client-side SSL credentials.
///
/// Use [`ChannelCredentialsBuilder`] or [`ChannelCredentials::google_default_credentials`] to
/// build a [`ChannelCredentials`].
pub struct ChannelCredentials {
    creds: *mut GrpcChannelCredentials,
}

impl ChannelCredentials {
    pub fn as_mut_ptr(&mut self) -> *mut GrpcChannelCredentials {
        self.creds
    }

    /// Try to build a [`ChannelCredentials`] to authenticate with Google OAuth credentials.
    pub fn google_default_credentials() -> Result<ChannelCredentials> {
        // Initialize the runtime here. Because this is an associated method
        // that can be called before construction of an `Environment`, we
        // need to call this here too.
        unsafe {
            grpc_sys::grpc_init();
        }
        let creds = unsafe { grpc_sys::grpc_google_default_credentials_create() };
        if creds.is_null() {
            Err(Error::GoogleAuthenticationFailed)
        } else {
            Ok(ChannelCredentials { creds })
        }
    }
}

impl Drop for ChannelCredentials {
    fn drop(&mut self) {
        unsafe { grpc_sys::grpc_channel_credentials_release(self.creds) }
    }
}
