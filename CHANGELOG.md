# 0.8.2 - 2012-03-10

- Fix send requirement in connectivity APIs (#516)

# 0.8.1 - 2021-03-05

- Support watch connectivity state (#513)
- Fix prost build of grpcio-proto (#515)

# grpcio-sys 0.8.1 - 2021-03-02

- Detect changes ahead to ease pain of upgrading compiler (#511)

# 0.8.0 - 2021-02-19

- Fix clippy warnings (#504)
- Add a way to not use bindgen (#499)
- Update gRPC C core to 1.35.0 (#506)
- Update bindgen to 0.57.0 (#507)

# 0.7.1 - 2020-12-18

- Allow CXX environment variable to override g++ for musl build (#500)
- Add user-defined checker for server side (#502)

# 0.7.0 - 2020-11-02

- Add blocking callback to `EnvBuilder` (#474)
- Enhance sinks to make them batchable (#469)
- Remove `rustfmt_skip` attribute since it is unstable (#479)
- Use `grpc_slice` to reduce memory copy (#481)
- Fix the bug that server cannot shutdown itself when drop (#484)
- Add methods for channels from file descriptors (#488)
- Update gRPC C core to 1.33.1 (#492)

# 0.6.0 - 2020-06-12

- Switch to std::future (#447)
- Update gRPC C core to 1.29.1 (#466)
- Change spinlock to parking_lot::Mutex (#468)

# 0.5.3 - 2020-05-07

- Switch to github action and update badge (#459)
- Enable ALPN by default (#456)

# grpcio-sys 0.5.2 - 2020-03-31

- Downgrade bindgen version to be backward compatible. (#452)

# 0.5.1 - 2020-03-30

- Clarify load balancing status (#445)
- Support unix domain socket (#446)
- Build: fix rebuild rules for no prebuilt bindings (#450)

# 0.5.0 - 2020-03-16

- Make `build_args` and `channel_args` public (#405)
- Reclaim buffer memory after sending message (#407)
- Support ppcle64 (#410)
- Use libz-sys instead of bundle one (#420)
- Update gRPC c core to v1.26.0 (#425)
- Support Authentication (#322)
- Update `Error` trait to latest version (#428)
- Update serveral outdated dependencies (#426)
- Better display and debug implement for status code and errors (#433, #437)
- Generate bindings for aarch64 target (#430)
- Support reloading certificates online (440)

# grpcio-compiler 0.5.0-alpha.6 - 2019-11-13

- Fix clippy warnings (#403)

# 0.5.0-alpha.5 - 2019-11-05

- Fix segment fault under race contention (#367)
- grpcio-compiler: remove protobuf-codegen dependency (#372)
- Add resource quota support (#377)
- Make metadata send (#363)
- Fix openssl link failure on Mac OS (#387)
- Fix compilation failure for latest gcc (#386)
- Fix deadlock when spawn multiple linked futures in the same queue (#395)

# 0.5.0-alpha.4 - 2019-08-12

- Make proto compile on Windows
- Make status code readable
- Remove clang requirement on x86_64 Linux

# 0.5.0-alpha.3 - 2019-07-24

- Fix circle dependencies to get round several cargo bugs
- Fix generating bindgen failure

# 0.5.0-alpha.2 - 2019-07-18

- Support using vendored openssl
- Use bindgen to generate code instead

# 0.5.0-alpha.1 - 2019-04-08

- Fix grpc_sys import when secure feature is disabled

# 0.5.0-alpha - 2019-04-03

- Support Prost
- Zero copy for receiving
- Support GrpcConnectivityState
- Upgrade to Rust 2018 edition

# 0.4.4 - 2019-02-15

- Support cross-compile for iOS and Android targets
- Support ipv6 host

# 0.4.3 - 2019-01-21

- Remove tilde requirements `~2.0` of protobuf

# 0.4.2 - 2019-01-07

- Update gRPC from 1.14.2 to 1.17.2

# 0.4.1 - 2018-11-15

- `Client` now is clonable
- Allow '.'s when validate metadata key
- Fix call validation issue when connection is closed
- Optionally use openssl instead of boring ssl

# 0.4.0 - 2018-09-15

- Update gRPC from 1.7.2 to 1.14.2
- Services accept mut reference
- Cancel RPC when senders and receivers were dropped
- Notify completion queue via call

# 0.3.1 - 2018-08-27

- Support configuring load balancing policy
- Fix compilation failure when go is missing
- Fix compilation issue under musl
- Fix soundness of service handler

# 0.3.0 - 2018-06-01

- keep compatible with protobuf 2.0
- enable secure feature by default
- fix potential overflow in channel args

# 0.2.3 - 2018-04-27

- support querying client address

# 0.2.2 - 2018-04-04

- use a different lock for notify to avoid deadlock

# 0.2.1 - 2018-02-23

- support ping configuration
- make `CallOptions` clonable
- support google default credentials
- fix link error on Windows
- support request header

# 0.2.0 - 2017-12-19

- update gRPC from 1.6.1 to 1.7.2
- separate secure/unsecure features
- fix compilation error on OS X and Win32
- publish gRPC built-in protos

# 0.1.2 - 2017-09-22

- use environment variable to control linking
- clear buffer hint when sending metadata

# 0.1.1 - 2017-09-21

- upgrade gRPC from 1.4.0 to 1.6.1
- support more channel args
- support log

# 0.1.0 - 2017-07-27

initial release
