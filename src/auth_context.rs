//! API for authenticating peer
//! Based on https://grpc.github.io/grpc/core/md_doc_server_side_auth.html

use std::ffi::CStr;
use std::marker::PhantomData;

use crate::grpc_sys::{
    self, grpc_auth_context, grpc_auth_property, grpc_auth_property_iterator, grpc_call,
};

/// To perform server-side authentication, gRPC exposes the authentication context
/// for each call. The context exposes important authentication-related information
/// about the RPC such as the type of security/authentication type being used and
/// the peer identity.
///
/// The authentication context is structured as a multi-map of key-value pairs -
/// the auth properties. In addition to that, for authenticated RPCs, the set of
/// properties corresponding to a selected key will represent the verified identity
/// of the caller - the peer identity.
///
/// The contents of the auth properties are populated by an auth interceptor within
/// gRPC Core. The interceptor also chooses which property key will act as the peer
/// identity (e.g. for client certificate authentication this property will be
/// `x509_common_name` or `x509_subject_alternative_name`).
pub struct AuthContext {
    ctx: *mut grpc_auth_context,
}

/// Binding to gRPC Core AuthContext
impl AuthContext {
    pub(crate) unsafe fn from_call_ptr(call: *mut grpc_call) -> Option<Self> {
        let ctx = grpc_sys::grpc_call_auth_context(call);
        if ctx.is_null() {
            None
        } else {
            Some(AuthContext { ctx })
        }
    }

    /// The name of the property gRPC Core has chosen as main peer identity property,
    /// if any.
    pub fn peer_identity_property_name(&self) -> Option<&str> {
        assert!(!self.ctx.is_null());
        unsafe {
            let p = grpc_sys::grpc_auth_context_peer_identity_property_name(self.ctx);
            if p.is_null() {
                None
            } else {
                Some(CStr::from_ptr(p).to_str().expect("valid UTF-8 data"))
            }
        }
    }

    /// `true` if the client has provided a valid certificate (or other auth method
    /// considered valid by gRPC).
    /// `false` in non-secure scenarios.
    pub fn peer_is_authenticated(&self) -> bool {
        assert!(!self.ctx.is_null());
        unsafe { grpc_sys::grpc_auth_context_peer_is_authenticated(self.ctx) != 0 }
    }

    /// `AuthContext[peer_identity_property_name()]`
    ///
    /// There may be several of them (for instance if `x509_subject_alternative_name` is selected)
    pub fn peer_identity(&self) -> AuthPropertyIter {
        assert!(!self.ctx.is_null());
        unsafe {
            // grpc_auth_context_peer_identity returns empty_iterator when self.ctx is NULL
            let iter = grpc_sys::grpc_auth_context_peer_identity(self.ctx);
            AuthPropertyIter {
                iter,
                _lifetime: PhantomData,
            }
        }
    }
}

impl<'a> IntoIterator for &'a AuthContext {
    type Item = AuthProperty<'a>;
    type IntoIter = AuthPropertyIter<'a>;

    /// Iterate over the AuthContext properties
    fn into_iter(self) -> Self::IntoIter {
        unsafe {
            // grpc_auth_context_property_iterator returns empty_iterator when self.ctx is NULL
            let iter = grpc_sys::grpc_auth_context_property_iterator(self.ctx);
            AuthPropertyIter {
                iter,
                _lifetime: PhantomData,
            }
        }
    }
}

impl<'a> Drop for AuthContext {
    fn drop(&mut self) {
        unsafe { grpc_sys::grpc_auth_context_release(self.ctx) }
    }
}

pub struct AuthPropertyIter<'a> {
    iter: grpc_auth_property_iterator,
    _lifetime: PhantomData<&'a grpc_auth_property_iterator>,
}

impl<'a> Iterator for AuthPropertyIter<'a> {
    type Item = AuthProperty<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // grpc_auth_property_iterator_next returns empty_iterator when self.iter is NULL
        let prop = unsafe {
            grpc_sys::grpc_auth_property_iterator_next(
                &mut self.iter as *mut grpc_auth_property_iterator,
            )
        };
        if prop.is_null() {
            None
        } else {
            Some(AuthProperty {
                prop,
                _lifetime: PhantomData,
            })
        }
    }
}

/// Auth properties are elements of the AuthContext. They have a name
/// (a key of type string) and a value which can be a string or binary data.
pub struct AuthProperty<'a> {
    prop: *const grpc_auth_property,
    _lifetime: PhantomData<&'a grpc_auth_property>,
}

impl<'a> AuthProperty<'a> {
    pub fn name(&self) -> &'a str {
        unsafe {
            CStr::from_ptr((*self.prop).name)
                .to_str()
                .expect("Auth property name should be valid UTF-8 data")
        }
    }

    pub fn value(&self) -> &'a [u8] {
        unsafe {
            std::slice::from_raw_parts((*self.prop).value as *const u8, (*self.prop).value_length)
        }
    }

    /// (name, value) tuple for easier pattern matching
    pub fn pair(&self) -> (&'a str, &'a [u8]) {
        (self.name(), self.value())
    }

    /// (name, value) tuple, but also trying to convert the value as an `&str`
    pub fn str_pair(&self) -> Result<(&'a str, &'a str), std::str::Utf8Error> {
        let value = std::str::from_utf8(self.value())?;
        Ok((self.name(), value))
    }
}
