#!/usr/bin/env bash
clang-tidy grpc-sys/grpc_wrap.cc -- -Igrpc-sys/grpc/include -x c++ -std=c++11
