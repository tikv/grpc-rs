extern crate gcc;

fn main() {
    // TODO: support static link
    gcc::Config::new()
        .file("grpc_wrap.c")
        .flag("-fPIC")
        .flag("-O2")
        .compile("libgrpc_wrap.a");

    println!("cargo:rustc-link-lib=grpc");
    println!("cargo:rustc-link-lib=grpc_wrap");
}
