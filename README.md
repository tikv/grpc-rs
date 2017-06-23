# gRPC-rs

[![Build Status](https://travis-ci.org/pingcap/grpc-rs.svg)](https://travis-ci.org/pingcap/grpc-rs)

The Rust wrapper of [gRPC Core](https://github.com/grpc/grpc). [Grpc](http://www.grpc.io) is a high performance, open source, general RPC framework that puts mobile and HTTP/2 first.

This project is still under developement, not all features are supported.

Status
------

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

Prerequisites
-------------

- Cmake
- Gcc (or Clang)
- Go (to build ssl support)
- Rust

Build
------------

```
$ cargo build --all
```

Performance
-----------
See [benchmark](https://github.com/pingcap/grpc-rs/tree/master/benchmark) to find out how to run a benchmark by yourself.
