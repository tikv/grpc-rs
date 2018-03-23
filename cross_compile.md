# Mac 下交叉编译 windows 版本 grpc-rs

修改 grpc-rs/grpc-sys/build.rs

```
# cmake 参数
# fix SYSTEM_PROCESSOR
"CMAKE_SYSTEM_PROCESSOR", "i686"
# fix try_run
"CMAKE_CROSSCOMPILING", "true"
```

修改 grpc-rs/grpc-sys/grpc/third_party/benchmark/cmake/CXXFeatureCheck.cmake

```
# 添加代码，fix try_run
SET( RUN_HAVE_STD_REGEX
     0
     CACHE STRING "Result from TRY_RUN" FORCE)

SET( RUN_HAVE_STEADY_CLOCK
     0
     CACHE STRING "Result from TRY_RUN" FORCE)
```

grpc-rs/grpc-sys/grpc/CMakeLists.txt
```
# 295 行左右
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -D_WIN32_WINNT=0x600")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -D_WIN32_WINNT=0x600")
set(C_CXX_FLAGS "${C_CXX_FLAGS} -D_WIN32_WINNT=0x600")
```

更新 boringssl 库到最新版，然后编译。
