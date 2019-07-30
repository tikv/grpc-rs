#!/bin/bash

# NOTE: 
# This script is only used when you want to generate bindings yourself.
# The generated bindings will overwrite grpc-sys/bindings/*

export UPDATE_BIND=1
cargo build
rustfmt grpc-sys/bindings/*