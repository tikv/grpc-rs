#!/bin/bash

# NOTE: 
# This script is only used when you want to generate bindings yourself.
# The generated bindings will overwrite grpc-sys/bindings/*

if [ "$ARCH" == "" ]; then
    ARCH=`uname -p`
fi
export UPDATE_BIND=1
cargo build -p grpcio-sys --target ${ARCH}-unknown-linux-gnu
rustfmt grpc-sys/bindings/*
if [ "$(uname -s)" == "Linux" ]; then
  sed -i '/^pub type .*= ::std::os::raw::.*/d' grpc-sys/bindings/*
fi
