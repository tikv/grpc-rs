git submodule update --init grpc-sys/grpc
cd grpc-sys/grpc
git submodule update --init third_party/cares/cares
git submodule update --init third_party/abseil-cpp
git submodule update --init third_party/re2
rm -rf third_party/boringssl-with-bazel/*
cd third_party/zlib
git clean -df
git reset --hard
