# gRPC-rs

[![Build Status](https://travis-ci.org/pingcap/grpc-rs.svg)](https://travis-ci.org/pingcap/grpc-rs)
[![Build status](https://ci.appveyor.com/api/projects/status/1cofa3nih5fm2kb0/branch/master?svg=true)](https://ci.appveyor.com/project/busyjay/grpc-rs/branch/master)

The Rust wrapper of [gRPC Core](https://github.com/grpc/grpc). [gRPC](http://www.grpc.io) is a high performance, open source, general RPC framework that puts mobile and HTTP/2 first.

Status
------
This project is still under developement, not all features are supported.

- [x] Basic asynchronous unary/steaming call
- [x] SSL
- [x] Generic call
- [x] Connetion level compression
- [x] Interoperability test
- [x] QPS benchmark
- [ ] Custom metadata
- [ ] Health check
- [ ] Reflection
- [ ] Authentication
- [ ] Load balance

Only linux and macOS are tested.

Prerequisites
-------------

- Cmake 3.8.0
- Gcc (or Clang)
- Go (to build ssl support) >=1.7
- Rust >= 1.19.0

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

2. Install gRPC compiler.

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
