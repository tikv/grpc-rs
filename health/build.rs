use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut builder = protobuf_build::Builder::new();
    builder
        .includes(&["proto".to_owned()])
        .files(&["proto/grpc/health/v1/health.proto"])
        .out_dir(&out_dir);

    #[cfg(feature = "prost-codec")]
    builder.wrapper_options(protobuf_build::GenOpt::empty());
    builder.generate();
}
