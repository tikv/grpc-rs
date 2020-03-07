use std::fs;
use std::io::{self, Read};

pub fn read_single_crt(name: &str) -> Result<String, io::Error> {
    let mut crt = String::new();
    fs::File::open(format!("certs/{}.crt", name))?.read_to_string(&mut crt)?;
    Ok(crt)
}

pub fn read_cert_pair(name: &str) -> Result<(String, String), io::Error> {
    let mut crt = String::new();
    let mut key = String::new();
    fs::File::open(format!("certs/{}.crt", name))?.read_to_string(&mut crt)?;
    fs::File::open(format!("certs/{}.key", name))?.read_to_string(&mut key)?;
    Ok((crt, key))
}
