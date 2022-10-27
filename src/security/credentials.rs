// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use std::error::Error as StdError;
use std::ffi::CString;
use std::{mem, ptr};

use crate::error::{Error, Result};
use crate::grpc_sys::grpc_ssl_certificate_config_reload_status::{self, *};
use crate::grpc_sys::grpc_ssl_client_certificate_request_type::*;
use crate::grpc_sys::{
    self, grpc_ssl_client_certificate_request_type, grpc_ssl_server_certificate_config,
};
use crate::{ChannelCredentials, ServerCredentials};

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CertificateRequestType {
    /// Server does not request client certificate.
    ///
    /// The certificate presented by the client is not checked by the server at
    /// all. (A client may present a self signed or signed certificate or not
    /// present a certificate at all and any of those option would be accepted)
    DontRequestClientCertificate = GRPC_SSL_DONT_REQUEST_CLIENT_CERTIFICATE as u32,
    /// Server requests client certificate but does not enforce that the client
    /// presents a certificate.
    ///
    /// If the client presents a certificate, the client authentication is left to
    /// the application (the necessary metadata will be available to the
    /// application via authentication context properties, see grpc_auth_context).
    ///
    /// The client's key certificate pair must be valid for the SSL connection to
    /// be established.
    RequestClientCertificateButDontVerify =
        GRPC_SSL_REQUEST_CLIENT_CERTIFICATE_BUT_DONT_VERIFY as u32,
    /// Server requests client certificate but does not enforce that the client
    /// presents a certificate.
    ///
    /// If the client presents a certificate, the client authentication is done by
    /// the gRPC framework. (For a successful connection the client needs to either
    /// present a certificate that can be verified against the root certificate
    /// configured by the server or not present a certificate at all)
    ///
    /// The client's key certificate pair must be valid for the SSL connection to
    /// be established.
    RequestClientCertificateAndVerify = GRPC_SSL_REQUEST_CLIENT_CERTIFICATE_AND_VERIFY as u32,
    /// Server requests client certificate and enforces that the client presents a
    /// certificate.
    ///
    /// If the client presents a certificate, the client authentication is left to
    /// the application (the necessary metadata will be available to the
    /// application via authentication context properties, see grpc_auth_context).
    ///
    /// The client's key certificate pair must be valid for the SSL connection to
    /// be established.
    RequestAndRequireClientCertificateButDontVerify =
        GRPC_SSL_REQUEST_AND_REQUIRE_CLIENT_CERTIFICATE_BUT_DONT_VERIFY as u32,
    /// Server requests client certificate and enforces that the client presents a
    /// certificate.
    ///
    /// The certificate presented by the client is verified by the gRPC framework.
    /// (For a successful connection the client needs to present a certificate that
    /// can be verified against the root certificate configured by the server)
    ///
    /// The client's key certificate pair must be valid for the SSL connection to
    /// be established.
    RequestAndRequireClientCertificateAndVerify =
        GRPC_SSL_REQUEST_AND_REQUIRE_CLIENT_CERTIFICATE_AND_VERIFY as u32,
}

/// Traits to retrieve updated SSL server certificates, private keys, and trusted CAs
/// (for client authentication).
pub trait ServerCredentialsFetcher {
    /// Retrieves updated credentials.
    ///
    /// The method will be called during server initialization and every time a new
    /// connection is about to be accepted. When returning `None` or error, gRPC
    /// will continue to use the previous certificates returned by the method. If no
    /// valid credentials is returned during initialization, the server will fail to start.
    fn fetch(&self) -> std::result::Result<Option<ServerCredentialsBuilder>, Box<dyn StdError>>;
}

impl CertificateRequestType {
    #[inline]
    pub(crate) fn to_native(self) -> grpc_ssl_client_certificate_request_type {
        unsafe { mem::transmute(self) }
    }
}

fn clear_key_securely(key: &mut [u8]) {
    unsafe {
        for b in key {
            ptr::write_volatile(b, 0)
        }
    }
}

pub(crate) unsafe extern "C" fn server_cert_fetcher_wrapper(
    user_data: *mut std::os::raw::c_void,
    config: *mut *mut grpc_ssl_server_certificate_config,
) -> grpc_ssl_certificate_config_reload_status {
    if user_data.is_null() {
        panic!("fetcher user_data must be set up!");
    }
    let f: &mut dyn ServerCredentialsFetcher =
        (*(user_data as *mut Box<dyn ServerCredentialsFetcher>)).as_mut();
    let result = f.fetch();
    match result {
        Ok(Some(builder)) => {
            let new_config = builder.build_config();
            *config = new_config;
        }
        Ok(None) => {
            return GRPC_SSL_CERTIFICATE_CONFIG_RELOAD_UNCHANGED;
        }
        Err(e) => {
            warn!("cert_fetcher met error: {}", e);
            return GRPC_SSL_CERTIFICATE_CONFIG_RELOAD_FAIL;
        }
    }
    GRPC_SSL_CERTIFICATE_CONFIG_RELOAD_NEW
}

/// [`ServerCredentials`] factory in order to configure the properties.
pub struct ServerCredentialsBuilder {
    root: Option<CString>,
    key_cert_pairs: Vec<grpcio_sys::grpc_ssl_pem_key_cert_pair>,
    cer_request_type: CertificateRequestType,
}

impl ServerCredentialsBuilder {
    /// Initialize a new [`ServerCredentialsBuilder`].
    pub fn new() -> ServerCredentialsBuilder {
        ServerCredentialsBuilder {
            root: None,
            key_cert_pairs: vec![],
            cer_request_type: CertificateRequestType::DontRequestClientCertificate,
        }
    }

    /// Set the PEM encoded client root certificate to verify client's identity. If
    /// `force_client_auth` is set to `true`, the authenticity of client check will be enforced.
    pub fn root_cert<S: Into<Vec<u8>>>(
        mut self,
        cert: S,
        cer_request_type: CertificateRequestType,
    ) -> ServerCredentialsBuilder {
        self.root = Some(CString::new(cert).unwrap());
        self.cer_request_type = cer_request_type;
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
        self.key_cert_pairs
            .push(grpcio_sys::grpc_ssl_pem_key_cert_pair {
                private_key: CString::new(private_key).unwrap().into_raw(),
                cert_chain: CString::new(cert).unwrap().into_raw(),
            });
        self
    }

    /// Finalize the [`ServerCredentialsBuilder`] and build the
    /// [`*mut grpcio_sys::bindings::grpc_ssl_server_certificate_config`].
    unsafe fn build_config(mut self) -> *mut grpcio_sys::grpc_ssl_server_certificate_config {
        let root_cert = self
            .root
            .take()
            .map_or_else(ptr::null_mut, CString::into_raw);
        let cfg = grpcio_sys::grpc_ssl_server_certificate_config_create(
            root_cert,
            self.key_cert_pairs.as_ptr(),
            self.key_cert_pairs.len(),
        );
        if !root_cert.is_null() {
            drop(CString::from_raw(root_cert));
        }
        cfg
    }

    /// Finalize the [`ServerCredentialsBuilder`] and build the [`ServerCredentials`].
    pub fn build(self) -> ServerCredentials {
        unsafe {
            let opt = grpcio_sys::grpc_ssl_server_credentials_create_options_using_config(
                self.cer_request_type.to_native(),
                self.build_config(),
            );
            let credentials = grpcio_sys::grpc_ssl_server_credentials_create_with_options(opt);
            ServerCredentials::from_raw(credentials)
        }
    }
}

impl Drop for ServerCredentialsBuilder {
    fn drop(&mut self) {
        for pair in self.key_cert_pairs.drain(..) {
            unsafe {
                drop(CString::from_raw(pair.cert_chain as *mut _));
                let s = CString::from_raw(pair.private_key as *mut _);
                clear_key_securely(&mut s.into_bytes_with_nul());
            }
        }
    }
}

impl ServerCredentials {
    /// Creates the credentials using a certificate config fetcher. Use this
    /// method to reload the certificates and keys of the SSL server without
    /// interrupting the operation of the server. Initial certificate config will be
    /// fetched during server initialization.
    pub fn with_fetcher(
        fetcher: Box<dyn ServerCredentialsFetcher + Send + Sync>,
        cer_request_type: CertificateRequestType,
    ) -> Self {
        let fetcher_wrap = Box::new(fetcher);
        let fetcher_wrap_ptr = Box::into_raw(fetcher_wrap);
        unsafe {
            let opt = grpcio_sys::grpc_ssl_server_credentials_create_options_using_config_fetcher(
                cer_request_type.to_native(),
                Some(server_cert_fetcher_wrapper),
                fetcher_wrap_ptr as _,
            );
            let mut creds = ServerCredentials::from_raw(
                grpcio_sys::grpc_ssl_server_credentials_create_with_options(opt),
            );
            creds._fetcher = Some(Box::from_raw(fetcher_wrap_ptr));
            creds
        }
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

        let mut pair = grpcio_sys::grpc_ssl_pem_key_cert_pair {
            private_key: key_ptr,
            cert_chain: cert_ptr,
        };
        let creds = unsafe {
            if cert_ptr.is_null() {
                grpcio_sys::grpc_ssl_credentials_create_ex(
                    root_ptr,
                    ptr::null_mut(),
                    ptr::null_mut(),
                    ptr::null_mut(),
                )
            } else {
                grpcio_sys::grpc_ssl_credentials_create_ex(
                    root_ptr,
                    &mut pair,
                    ptr::null_mut(),
                    ptr::null_mut(),
                )
            }
        };

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

impl ChannelCredentials {
    /// Try to build a [`ChannelCredentials`] to authenticate with Google OAuth credentials.
    pub fn google_default_credentials() -> Result<ChannelCredentials> {
        // Initialize the runtime here. Because this is an associated method
        // that can be called before construction of an `Environment`, we
        // need to call this here too.
        unsafe {
            grpc_sys::grpc_init();
        }
        let creds = unsafe { grpc_sys::grpc_google_default_credentials_create(ptr::null_mut()) };
        if creds.is_null() {
            Err(Error::GoogleAuthenticationFailed)
        } else {
            Ok(ChannelCredentials { creds })
        }
    }
}
