# gRPC-rs

[![Build Status](https://travis-ci.org/pingcap/grpc-rs.svg)](https://travis-ci.org/pingcap/grpc-rs)

The Rust wrapper of [gRPC Core](https://github.com/grpc/grpc). [gRPC](http://www.grpc.io) is a high performance, open source, general RPC framework that puts mobile and HTTP/2 first.

Status
------
This project is still under developement, not all features are supported.

- [x] basic async unary/steaming call
- [x] ssl
- [x] generic call
- [x] connetion level compression
- [x] interop test
- [x] qps benchmark
- [ ] custom metadata
- [ ] health check
- [ ] reflection
- [ ] auth
- [ ] load balance

Only linux and macOS are tested.

Prerequisites
-------------

- Cmake 3.8.0
- Gcc (or Clang)
- Go (to build ssl support) >=1.7
- Rust >= 1.18.0

Build
-----

```
$ cargo build --all
```

Usage
-----

To generate the sources from proto files:

1. Install protobuf compiler.

```
$ cargo install protobuf
```

2. Install grpc compiler.

```
$ cargo install --git https://github.com/pingcap/grpc-rs.git grpc-compiler
```

3. Generate sources.

```
$ protoc --rust_out=. --grpc_out=. --plugin=protoc-gen-grpc=`which grpc_rust_plugin` example.proto
```

To include this project as a dependency:

```
[dependencies.grpc]
git = "https://github.com/pingcap/grpc-rs.git"
```

Performance
-----------
See [benchmark](https://github.com/pingcap/grpc-rs/tree/master/benchmark) to find out how to run a benchmark by yourself.
