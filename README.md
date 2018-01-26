# gRPC-rs

`gRPC-rs` is a Rust wrapper of [gRPC Core](https://github.com/grpc/grpc). [gRPC](http://www.grpc.io) is a high performance, open source universal RPC framework that puts mobile and HTTP/2 first.

[![Crates.io](https://img.shields.io/crates/v/grpcio.svg?maxAge=2592000)](https://crates.io/crates/grpcio)
[![Build Status](https://travis-ci.org/pingcap/grpc-rs.svg)](https://travis-ci.org/pingcap/grpc-rs)
[![Build status](https://ci.appveyor.com/api/projects/status/1cofa3nih5fm2kb0/branch/master?svg=true)](https://ci.appveyor.com/project/busyjay/grpc-rs/branch/master)

Status
------
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
- [ ] Authentication
- [ ] Load balance

Prerequisites
-------------

- CMake >= 3.8.0
- Rust >= 1.19.0
- If you want to enable secure feature, Go (>=1.7) is required.

For Linux and MacOS, you also need to install gcc (or clang) too.

For Windows, you also need to install following software:

- Active State Perl 
- yasm
- Visual Studio 2015+

Build
-----

```
$ cargo build
```

Usage
-----

To generate the sources from proto files:

Option 1 - Manual Generation
----------------------------

1. Install the protobuf compiler:

```
$ cargo install protobuf
```

2. Install the gRPC compiler:

```
$ cargo install grpcio-compiler
```

3. Generate the sources:

```
$ protoc --rust_out=. --grpc_out=. --plugin=protoc-gen-grpc=`which grpc_rust_plugin` example.proto
```

To include this project as a dependency:

```
[dependencies]
grpcio = "0.1"
```

Option 2 - Programmatic Generation
----------------------------------

Programmatic generation can be used to generate Rust modules from proto files
via your `build.rs` by using [protoc-grpcio](https://crates.io/crates/protoc-grpcio).

Include this in your `Cargo.toml`:

```toml
[build.dependencies]
protoc-grpcio = "0.1"
```

and then use in your `build.rs` like:

```rust
extern crate protoc_grpcio;

// Generates `protobuf.rs` and `protobuf_grpc.rs` in a directory named
// `output`.
protoc_grpcio::compile_grpc_protos(
    &["example/protobuf.proto"],
    &["example"],
    "output"
).expect("failed to compile gRPC definitions");
```

Performance
-----------
See [benchmark](https://github.com/pingcap/grpc-rs/tree/master/benchmark) to find out how to run a benchmark by yourself.
