// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use std::error::Error as StdError;
use std::ffi::CString;
use std::{mem, ptr};

use crate::error::{Error, Result};
use crate::grpc_sys::grpc_ssl_certificate_config_reload_status::{self, *};
use crate::grpc_sys::grpc_ssl_client_certificate_request_type::*;
use crate::grpc_sys::{
    self, grpc_channel_credentials, grpc_server_credentials,
    grpc_ssl_client_certificate_request_type, grpc_ssl_server_certificate_config,
};

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

/// User-provided callback function for reload cert. [`None`]
/// indicates that no reloading is needed, and
/// [`Some(ServerCredentialsBuilder)`] indicates that reloading
/// is needed.
pub type CertFetcher =
    fn() -> std::result::Result<Option<ServerCredentialsBuilder>, Box<dyn StdError>>;

#[repr(C)]
pub struct CertUsrData {
    initial_cert_cfg: *mut grpc_ssl_server_certificate_config,
    cert_fetcher: CertFetcher,
    initial_cert_fetched: bool,
}

impl CertificateRequestType {
    #[inline]
    fn to_native(self) -> grpc_ssl_client_certificate_request_type {
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

extern "C" fn server_cert_fetcher_wrapper(
    user_data: *mut std::os::raw::c_void,
    config: *mut *mut grpc_ssl_server_certificate_config,
) -> grpc_ssl_certificate_config_reload_status {
    if user_data.is_null() {
        panic!("fetcher user_data must be set up!");
    }
    let usr_data: &mut CertUsrData = unsafe { &mut *(user_data as *mut CertUsrData) };
    let mut _cert_config = std::ptr::null_mut();

    if !usr_data.initial_cert_fetched {
        _cert_config = usr_data.initial_cert_cfg;
        usr_data.initial_cert_cfg = std::ptr::null_mut();
        usr_data.initial_cert_fetched = true;
    } else {
        let result = (usr_data.cert_fetcher)();

        match result {
            Err(e) => {
                warn!("cert_fetcher met some errors: {}", e);
                return GRPC_SSL_CERTIFICATE_CONFIG_RELOAD_FAIL;
            }
            Ok(ret) => {
                if let Some(mut builder) = ret {
                    let root_cert = builder
                        .root
                        .take()
                        .map_or_else(ptr::null_mut, CString::into_raw);
                    let new_config = unsafe {
                        grpcio_sys::grpc_ssl_server_certificate_config_create(
                            root_cert,
                            builder.key_cert_pairs.as_ptr(),
                            builder.key_cert_pairs.len(),
                        )
                    };
                    _cert_config = new_config;
                } else {
                    return GRPC_SSL_CERTIFICATE_CONFIG_RELOAD_UNCHANGED;
                }
            }
        }
    }
    unsafe {
        *config = _cert_config;
    }
    GRPC_SSL_CERTIFICATE_CONFIG_RELOAD_NEW
}

/// [`ServerCredentials`] factory in order to configure the properties.
pub struct ServerCredentialsBuilder {
    root: Option<CString>,
    key_cert_pairs: Vec<grpcio_sys::grpc_ssl_pem_key_cert_pair>,
    cer_request_type: CertificateRequestType,
    force_client_auth: bool,
    cert_usr_data: Option<Box<CertUsrData>>,
}

impl ServerCredentialsBuilder {
    /// Initialize a new [`ServerCredentialsBuilder`].
    pub fn new() -> ServerCredentialsBuilder {
        ServerCredentialsBuilder {
            root: None,
            key_cert_pairs: vec![],
            cer_request_type: CertificateRequestType::DontRequestClientCertificate,
            force_client_auth: false,
            cert_usr_data: None,
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

    pub fn set_force_client_auth(mut self, b: bool) -> ServerCredentialsBuilder {
        self.force_client_auth = b;
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

    /// Add the fullstack user fetcher for reloading certificates.
    ///
    /// WARN: Do Not Call [`build`] Method In Your Fetcher!
    /// ServerCredentialsBuilder in fetcher is only used to be creating cert config.
    pub fn add_reload_fetcher(mut self, fetcher: CertFetcher) -> ServerCredentialsBuilder {
        let data = Box::new(CertUsrData {
            initial_cert_cfg: std::ptr::null_mut(),
            cert_fetcher: fetcher,
            initial_cert_fetched: false,
        });
        self.cert_usr_data = Some(data);
        self
    }

    /// Finalize the [`ServerCredentialsBuilder`] and build the [`ServerCredentials`].
    pub fn build(mut self) -> ServerCredentials {
        let root_cert = self
            .root
            .take()
            .map_or_else(ptr::null_mut, CString::into_raw);
        unsafe {
            if self.cert_usr_data.is_some() {
                let mut data = self.cert_usr_data.take().unwrap();
                let initial_cert_cfg = grpcio_sys::grpc_ssl_server_certificate_config_create(
                    root_cert,
                    self.key_cert_pairs.as_ptr(),
                    self.key_cert_pairs.len(),
                );
                data.initial_cert_cfg = initial_cert_cfg;
                let p_data = Box::into_raw(data);
                let opt = grpc_sys::grpc_ssl_server_credentials_create_options_using_config_fetcher(
                    if self.force_client_auth {
                        GRPC_SSL_REQUEST_AND_REQUIRE_CLIENT_CERTIFICATE_AND_VERIFY
                    } else {
                        self.cer_request_type.to_native()
                    },
                    Some(server_cert_fetcher_wrapper),
                    p_data as _,
                );
                let credentials = grpcio_sys::grpc_ssl_server_credentials_create_with_options(opt);
                ServerCredentials {
                    c_creds: credentials,
                    cert_user_data: Some(Box::from_raw(p_data)),
                }
            } else {
                let cfg = grpcio_sys::grpc_ssl_server_certificate_config_create(
                    root_cert,
                    self.key_cert_pairs.as_ptr(),
                    self.key_cert_pairs.len(),
                );
                let opt = grpcio_sys::grpc_ssl_server_credentials_create_options_using_config(
                    if self.force_client_auth {
                        GRPC_SSL_REQUEST_AND_REQUIRE_CLIENT_CERTIFICATE_AND_VERIFY
                    } else {
                        self.cer_request_type.to_native()
                    },
                    cfg,
                );
                let credentials = grpcio_sys::grpc_ssl_server_credentials_create_with_options(opt);

                if !root_cert.is_null() {
                    CString::from_raw(root_cert);
                }

                ServerCredentials {
                    c_creds: credentials,
                    cert_user_data: None,
                }
            }
        }
    }
}

impl Drop for ServerCredentialsBuilder {
    fn drop(&mut self) {
        for pair in self.key_cert_pairs.drain(..) {
            unsafe {
                CString::from_raw(pair.cert_chain as *mut _);
                let s = CString::from_raw(pair.private_key as *mut _);
                clear_key_securely(&mut s.into_bytes_with_nul());
            }
        }
    }
}

/// Server-side SSL credentials.
///
/// Use [`ServerCredentialsBuilder`] to build a [`ServerCredentials`].
pub struct ServerCredentials {
    c_creds: *mut grpc_server_credentials,
    cert_user_data: Option<Box<CertUsrData>>,
}

impl ServerCredentials {
    pub fn as_mut_ptr(&mut self) -> *mut grpc_server_credentials {
        self.c_creds
    }

    pub fn take_cert_user_data(&mut self) -> Option<Box<CertUsrData>> {
        self.cert_user_data.take()
    }
}

impl Drop for ServerCredentials {
    fn drop(&mut self) {
        if !self.c_creds.is_null() {
            unsafe {
                grpc_sys::grpc_server_credentials_release(self.c_creds);
            }
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

/// Client-side SSL credentials.
///
/// Use [`ChannelCredentialsBuilder`] or [`ChannelCredentials::google_default_credentials`] to
/// build a [`ChannelCredentials`].
pub struct ChannelCredentials {
    creds: *mut grpc_channel_credentials,
}

impl ChannelCredentials {
    pub fn as_mut_ptr(&mut self) -> *mut grpc_channel_credentials {
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
