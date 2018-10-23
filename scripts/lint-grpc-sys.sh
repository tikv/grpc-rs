#!/usr/bin/env bash
clang-tidy-5.0 grpc-sys/grpc_wrap.cc -- -Igrpc-sys/grpc/include -x c++ -std=c++11
