#!/usr/bin/env bash
clang-tidy-5.0 grpc-sys/grpc_wrap.cc -checks=clang-analyzer-* \
 -- -Igrpc-sys/grpc/include -x c++ -std=c++11
