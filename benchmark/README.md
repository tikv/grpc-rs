For benchmark design details, see [benchmarking](http://www.grpc.io/docs/guides/benchmarking.html).

Quick Start
===========

1. Clone grpc-rs

```
$ git clone https://github.com/tikv/grpc-rs.git
```

2. Clone grpc

```
$ git clone https://github.com/pingcap/grpc.git
```

3. Build benchmark

`grpc` and `grpc-rs` need to be in the same folder
```
$ cd grpc-rs
$ cargo xtask submodule
$ cargo build -p benchmark --release
```

4. Run benchmark

```
$ cd ../grpc
$ git submodule update --init
$ python3 tools/run_tests/run_performance_tests.py -l rust
```
The recommended python version is 3.10

Checkout `python3 tools/run_tests/run_performance_tests.py --help` to see custom options.

Flame Graph
===========

To generate flame graph, please download FrameGraph release package and extract them in grpc directory.
Please make sure the name of extracted directory is FlameGraph. Then run following command:

```
# python3 tools/run_tests/run_performance_tests.py -l rust --perf_args="record -F 99 -g"
```
