git submodule update --init grpc-sys/grpc
cd grpc-sys/grpc
git submodule update --init third_party/boringssl-with-bazel
git submodule update --init third_party/cares/cares
git submodule update --init third_party/abseil-cpp
cd third_party/zlib
git clean -f
git reset --hard
