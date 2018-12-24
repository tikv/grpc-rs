# Cross Compile gRPC-rs(0.2.1) to Windows under *nix

## First you need to install mingw

```bash
# macOS
brew install mingw-w64

# CentOS
yum install mingw64-openssl-static mingw64-zlib-static mingw64-winpthreads-static
```

## Fix CMake

```
# modify grpc-rs/grpc-sys/build.rs
# fix SYSTEM_PROCESSOR
"CMAKE_SYSTEM_PROCESSOR", get_env("CARGO_CFG_TARGET_ARCH").unwrap()
# fix try_run
"CMAKE_CROSSCOMPILING", "true"
```

### All diff in `fn build_grpc`

```rust
    let dst = {
        let mut config = Config::new("grpc");
        if get_env("CARGO_CFG_TARGET_OS").map_or(false, |s| s == "macos") {
            config.cxxflag("-stdlib=libc++");
        }
        config
            .define("CMAKE_SYSTEM_PROCESSOR", get_env("CARGO_CFG_TARGET_ARCH").unwrap())
            .define("CMAKE_CROSSCOMPILING", "true")
            .build_target(library)
            .uses_cxx11()
            .build()
        // config.build_target(library).uses_cxx11().build()
    };
```

### Fix find zlib

```rust
    // try these values
    let mut zlib = "z";
    let mut zlib = "zlibstatic";
    let mut zlib = "zlibstaticd";
```

## Fix try_run

```
# grpc-rs/grpc-sys/grpc/third_party/benchmark/cmake/CXXFeatureCheck.cmake
# add these code to fix try_run
SET( RUN_HAVE_STD_REGEX
     0
     CACHE STRING "Result from TRY_RUN" FORCE)

SET( RUN_HAVE_STEADY_CLOCK
     0
     CACHE STRING "Result from TRY_RUN" FORCE)
```

## Fix WIN32 API

```
# grpc-rs/grpc-sys/grpc/CMakeLists.txt
# add these code after about line number 295
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -D_WIN32_WINNT=0x600")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -D_WIN32_WINNT=0x600")
set(C_CXX_FLAGS "${C_CXX_FLAGS} -D_WIN32_WINNT=0x600")
```

## Fix boringssl

Just update third_party/boringssl

```bash
cd third_party/boringssl
git checkout master
git pull
```
