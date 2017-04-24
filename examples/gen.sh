#!/bin/sh
set -ex

protoc --rust_out=generated route_guide.proto
protoc --rust-grpc_out=generated route_guide.proto
