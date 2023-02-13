// Copyright 2020 TiKV Project Authors. Licensed under Apache-2.0.

use std::fs;
use std::io::{self, Read};

const CERT_DIR: &str = "../grpc-sys/grpc/src/core/tsi/test_creds";

fn cert_path(name: &str, ext: &str) -> String {
    let p = format!("{CERT_DIR}/{name}.{ext}");
    println!("reading {p}");
    p
}

pub fn read_single_crt(name: &str) -> Result<String, io::Error> {
    let mut crt = String::new();
    fs::File::open(cert_path(name, "pem"))?.read_to_string(&mut crt)?;
    Ok(crt)
}

pub fn read_cert_pair(name: &str) -> Result<(String, String), io::Error> {
    let mut crt = String::new();
    let mut key = String::new();
    fs::File::open(cert_path(name, "pem"))?.read_to_string(&mut crt)?;
    fs::File::open(cert_path(name, "key"))?.read_to_string(&mut key)?;
    Ok((crt, key))
}
