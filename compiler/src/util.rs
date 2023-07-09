// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use std::fmt;
use std::str;

// A struct that divide a name into serveral parts that meets rust's guidelines.
struct NameSpliter<'a> {
    name: &'a [u8],
    pos: usize,
}

impl<'a> NameSpliter<'a> {
    fn new(s: &str) -> NameSpliter {
        NameSpliter {
            name: s.as_bytes(),
            pos: 0,
        }
    }
}

impl<'a> Iterator for NameSpliter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        if self.pos == self.name.len() {
            return None;
        }
        // skip all prefix '_'
        while self.pos < self.name.len() && self.name[self.pos] == b'_' {
            self.pos += 1;
        }
        let mut pos = self.name.len();
        let mut upper_len = 0;
        let mut meet_lower = false;
        for i in self.pos..self.name.len() {
            let c = self.name[i];
            if c.is_ascii_uppercase() {
                if meet_lower {
                    // So it should be AaA or aaA
                    pos = i;
                    break;
                }
                upper_len += 1;
            } else if c == b'_' {
                pos = i;
                break;
            } else {
                meet_lower = true;
                if upper_len > 1 {
                    // So it should be AAa
                    pos = i - 1;
                    break;
                }
            }
        }
        let s = str::from_utf8(&self.name[self.pos..pos]).unwrap();
        self.pos = pos;
        Some(s)
    }
}

/// Adjust method name to follow rust-guidelines.
pub fn to_snake_case(name: &str) -> String {
    let mut snake_method_name = String::with_capacity(name.len());
    for s in NameSpliter::new(name) {
        snake_method_name.push_str(&s.to_lowercase());
        snake_method_name.push('_');
    }
    snake_method_name.pop();
    snake_method_name
}

#[cfg(feature = "protobuf-codec")]
pub fn to_camel_case(name: &str) -> String {
    let mut camel_case_name = String::with_capacity(name.len());
    for s in NameSpliter::new(name) {
        let mut chs = s.chars();
        camel_case_name.extend(chs.next().unwrap().to_uppercase());
        camel_case_name.push_str(&s[1..].to_lowercase());
    }
    camel_case_name
}

pub fn fq_grpc(item: &str) -> String {
    format!("::grpcio::{item}")
}

pub enum MethodType {
    Unary,
    ClientStreaming,
    ServerStreaming,
    Duplex,
}

impl fmt::Display for MethodType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MethodType::Unary => "MethodType::Unary",
                MethodType::ClientStreaming => "MethodType::ClientStreaming",
                MethodType::ServerStreaming => "MethodType::ServerStreaming",
                MethodType::Duplex => "MethodType::Duplex",
            }
        )
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_snake_name() {
        let cases = vec![
            ("AsyncRequest", "async_request"),
            ("asyncRequest", "async_request"),
            ("async_request", "async_request"),
            ("createID", "create_id"),
            ("AsyncRClient", "async_r_client"),
            ("CreateIDForReq", "create_id_for_req"),
            ("Create_ID_For_Req", "create_id_for_req"),
            ("Create_ID_For__Req", "create_id_for_req"),
            ("ID", "id"),
            ("id", "id"),
        ];

        for (origin, exp) in cases {
            let res = super::to_snake_case(origin);
            assert_eq!(res, exp);
        }
    }

    #[test]
    #[cfg(feature = "protobuf-codec")]
    fn test_camel_name() {
        let cases = vec![
            ("AsyncRequest", "AsyncRequest"),
            ("asyncRequest", "AsyncRequest"),
            ("async_request", "AsyncRequest"),
            ("createID", "CreateId"),
            ("AsyncRClient", "AsyncRClient"),
            ("async_r_client", "AsyncRClient"),
            ("CreateIDForReq", "CreateIdForReq"),
            ("Create_ID_For_Req", "CreateIdForReq"),
            ("Create_ID_For__Req", "CreateIdForReq"),
            ("ID", "Id"),
            ("id", "Id"),
        ];

        for (origin, exp) in cases {
            let res = super::to_camel_case(origin);
            assert_eq!(res, exp);
        }
    }
}
