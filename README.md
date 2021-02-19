# gRPC-rs

`gRPC-rs` is a Rust wrapper of [gRPC Core](https://github.com/grpc/grpc). [gRPC](http://www.grpc.io) is a high performance, open source universal RPC framework that puts mobile and HTTP/2 first.

[![Crates.io](https://img.shields.io/crates/v/grpcio.svg?maxAge=2592000)](https://crates.io/crates/grpcio)
[![docs.rs](https://docs.rs/grpcio/badge.svg)](https://docs.rs/grpcio)
[![Build Status](https://github.com/tikv/grpc-rs/workflows/CI/badge.svg)](https://github.com/tikv/grpc-rs/actions)
[![Build Status](https://travis-ci.org/tikv/grpc-rs.svg)](https://travis-ci.org/tikv/grpc-rs)

## Status

This project is still under development. The following features with the check marks are supported:

- [x] Basic asynchronous unary/steaming call
- [x] SSL
- [x] Generic call
- [x] Connection level compression
- [x] Interoperability test
- [x] QPS benchmark
- [ ] Custom metadata
- [x] Health check
- [ ] Reflection
- [X] Authentication
- [ ] Load balance, client side is fully supported, server side load report is not implemented yet.

## Prerequisites

- CMake >= 3.8.0
- Rust >= 1.36.0
- binutils >= 2.22
- LLVM and Clang >= 3.9 if you need to generate bindings at compile time.
- By default, the [secure feature](#feature-secure) is provided by boringssl. You can also use openssl instead by enabling [openssl feature](#feature-openssl).

For Linux and MacOS, you also need to install gcc 4.9+ (or clang) too.

Bindings are pre-generated for x86_64/arm64 Linux. For other platforms, bindings are generated at compile time.

For Windows, you also need to install following software:

- Active State Perl
- yasm
- Visual Studio 2015+

## Build

```
$ ./scripts/reset-submodule.cmd # if you just cloned the repository
$ cargo build
```

### Error linking OpenSSL

If you're getting linker errors when building your project using `gRPC-rs`, head
down to the `openssl` feature section for a possible fix.

## Usage

To generate the sources from proto files:

### Option 1 - Manual Generation

1. Install the protobuf compiler:

```
$ cargo install protobuf-codegen
```

2. Install the gRPC compiler:

```
$ cargo install grpcio-compiler
```

3. Generate the sources:

```
$ protoc --rust_out=. --grpc_out=. --plugin=protoc-gen-grpc=`which grpc_rust_plugin` example.proto
```


### Option 2 - Programmatic Generation

Programmatic generation can be used to generate Rust modules from proto files
via your `build.rs` by using [protoc-grpcio](https://crates.io/crates/protoc-grpcio).

For more information and examples see
[README](https://github.com/mtp401/protoc-grpcio/blob/master/README.md).

To include this project as a dependency:

```
[dependencies]
grpcio = "0.6"
```

### Feature `secure`

`secure` feature enables support for TLS encryption and some authentication
mechanism. When you do not need it, for example when working in intranet,
you can disable it by using the following configuration:
```
[dependencies]
grpcio = { version = "0.6", default-features = false, features = ["protobuf-codec"] }
```

### Feature `prost-codec` and `protobuf-codec`

`gRPC-rs` uses `protobuf` crate by default. If you want to use `prost` instead, you can enable
`prost-codec` feature. You probably only want to enable only one of the two features. Though
grpcio is completely fine with both features enabled at the same time, grpcio-compiler
will not going to work as expected.

### Feature `openssl` and `openssl-vendored`

`gRPC-rs` comes vendored with `gRPC Core`, which by default uses BoringSSL
instead of OpenSSL. This may cause linking issues due to symbol clashes and/or
missing symbols when another one of your dependencies uses OpenSSL. To resolve
this, you can tell `gRPC-rs` to use OpenSSL too by specifying `"openssl"` in
your `Cargo.toml`'s features list for `gprcio`, which requires openssl (>=1.0.2). E.g.:

```toml
[dependencies]
grpcio = { version = "0.6", features = ["openssl"] }
```

Feature `openssl-vendored` is the same as feature `openssl` except it will build openssl from
bundled sources.

## Performance

See [benchmark](https://github.com/tikv/grpc-rs/tree/master/benchmark) to find out how to run a benchmark by yourself.

Cross Compile
-------------
See [cross_compile](cross_compile.md)

Contributing
------------

Make sure to format and test the code before sending a PR.

If the content in grpc-sys/grpc is updated, you may need to regenerate bindings:

```
$ ./scripts/generate-bindings.sh
```
