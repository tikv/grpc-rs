#!/bin/bash

# NOTE: 
# This script is only used when you want to generate bindings yourself.
# The generated bindings will overwrite grpc-sys/bindings/*

export UPDATE_BIND=1
cargo build -p grpcio-sys --target x86_64-unknown-linux-gnu
rustfmt grpc-sys/bindings/*
