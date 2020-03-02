// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

extern crate cc;
extern crate cmake;
extern crate pkg_config;

use std::env::VarError;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

use cc::Build;
use cmake::Config;
use pkg_config::{Config as PkgConfig, Library};
use walkdir::WalkDir;

const GRPC_VERSION: &str = "1.26.0";

fn probe_library(library: &str, cargo_metadata: bool) -> Library {
    match PkgConfig::new()
        .atleast_version(GRPC_VERSION)
        .cargo_metadata(cargo_metadata)
        .probe(library)
    {
        Ok(lib) => lib,
        Err(e) => panic!("can't find library {} via pkg-config: {:?}", library, e),
    }
}

fn prepare_grpc() {
    let mut modules = vec![
        "grpc",
        "grpc/third_party/cares/cares",
        "grpc/third_party/address_sorting",
    ];

    if cfg!(feature = "secure") {
        modules.push("grpc/third_party/boringssl");
    }

    for module in modules {
        if is_directory_empty(module).unwrap_or(true) {
            panic!(
                "Can't find module {}. You need to run `git submodule \
                 update --init --recursive` first to build the project.",
                module
            );
        }
    }
}

fn is_directory_empty<P: AsRef<Path>>(p: P) -> Result<bool, io::Error> {
    let mut entries = fs::read_dir(p)?;
    Ok(entries.next().is_none())
}

fn trim_start<'a>(s: &'a str, prefix: &str) -> Option<&'a str> {
    if s.starts_with(prefix) {
        Some(s.trim_start_matches(prefix))
    } else {
        None
    }
}

fn build_grpc(cc: &mut Build, library: &str) {
    prepare_grpc();

    let mut third_party = vec!["cares/cares/lib"];

    let dst = {
        let mut config = Config::new("grpc");
        if !cfg!(feature = "secure") {
            // boringssl's configuration is still included, but targets
            // will never be built, hence specify a fake go to get rid of
            // the unnecessary dependency.
            config.define("GO_EXECUTABLE", "fake-go-nonexist");
        }
        if get_env("CARGO_CFG_TARGET_OS").map_or(false, |s| s == "macos") {
            config.cxxflag("-stdlib=libc++");
        }
        if env::var("CARGO_CFG_TARGET_ENV").unwrap() == "musl" {
            config.define("CMAKE_CXX_COMPILER", "g++");
        }

        // Cross-compile support for iOS
        match env::var("TARGET").unwrap().as_str() {
            "aarch64-apple-ios" => {
                config
                    .define("CMAKE_OSX_SYSROOT", "iphoneos")
                    .define("CMAKE_OSX_ARCHITECTURES", "arm64");
            }
            "armv7-apple-ios" => {
                config
                    .define("CMAKE_OSX_SYSROOT", "iphoneos")
                    .define("CMAKE_OSX_ARCHITECTURES", "armv7");
            }
            "armv7s-apple-ios" => {
                config
                    .define("CMAKE_OSX_SYSROOT", "iphoneos")
                    .define("CMAKE_OSX_ARCHITECTURES", "armv7s");
            }
            "i386-apple-ios" => {
                config
                    .define("CMAKE_OSX_SYSROOT", "iphonesimulator")
                    .define("CMAKE_OSX_ARCHITECTURES", "i386");
            }
            "x86_64-apple-ios" => {
                config
                    .define("CMAKE_OSX_SYSROOT", "iphonesimulator")
                    .define("CMAKE_OSX_ARCHITECTURES", "x86_64");
            }
            _ => {}
        };

        // Allow overriding of the target passed to cmake
        // (needed for Android crosscompile)
        if let Ok(val) = env::var("CMAKE_TARGET_OVERRIDE") {
            config.target(&val);
        }

        // We don't need to generate install targets.
        config.define("gRPC_INSTALL", "false");
        // We don't need to build csharp target.
        config.define("gRPC_BUILD_CSHARP_EXT", "false");
        // We don't need to build codegen target.
        config.define("gRPC_BUILD_CODEGEN", "false");
        // We don't need to build benchmarks.
        config.define("gRPC_BENCHMARK_PROVIDER", "none");
        if cfg!(feature = "openssl") {
            config.define("gRPC_SSL_PROVIDER", "package");
            config.define("EMBED_OPENSSL", "false");
            // Problem is: Ubuntu Trusty shipped with openssl 1.0.1f. Which doesn't
            // support alpn. And Google's gRPC checks for support of ALPN in plane
            // old Makefile, but not in CMake.
            config.cxxflag("-DTSI_OPENSSL_ALPN_SUPPORT=0");
            setup_openssl(&mut config)
        } else if cfg!(feature = "secure") {
            third_party.extend_from_slice(&["boringssl/ssl", "boringssl/crypto"]);
        }
        if cfg!(feature = "no-omit-frame-pointer") {
            config
                .cflag("-fno-omit-frame-pointer")
                .cxxflag("-fno-omit-frame-pointer");
        }
        // Uses zlib from libz-sys.
        setup_libz(&mut config);
        config.build_target(library).uses_cxx11().build()
    };

    let build_dir = format!("{}/build", dst.display());
    if get_env("CARGO_CFG_TARGET_OS").map_or(false, |s| s == "windows") {
        let profile = match &*env::var("PROFILE").unwrap() {
            "bench" | "release" => "Release",
            _ => "Debug",
        };
        println!("cargo:rustc-link-search=native={}/{}", build_dir, profile);
        for path in third_party {
            println!(
                "cargo:rustc-link-search=native={}/third_party/{}/{}",
                build_dir, path, profile
            );
        }
    } else {
        println!("cargo:rustc-link-search=native={}", build_dir);
        for path in third_party {
            println!(
                "cargo:rustc-link-search=native={}/third_party/{}",
                build_dir, path,
            );
        }
    }

    println!("cargo:rustc-link-lib=static=z");
    println!("cargo:rustc-link-lib=static=cares");
    println!("cargo:rustc-link-lib=static=gpr");
    println!("cargo:rustc-link-lib=static=address_sorting");
    println!("cargo:rustc-link-lib=static=upb");
    println!("cargo:rustc-link-lib=static={}", library);

    if cfg!(feature = "secure") {
        if cfg!(feature = "openssl") && !cfg!(feature = "openssl-vendored") {
            figure_ssl_path(&build_dir);
        } else {
            println!("cargo:rustc-link-lib=static=ssl");
            println!("cargo:rustc-link-lib=static=crypto");
        }
    }

    cc.include("grpc/include");
}

fn figure_ssl_path(build_dir: &str) {
    let path = format!("{}/CMakeCache.txt", build_dir);
    let f = BufReader::new(std::fs::File::open(&path).unwrap());
    let mut cnt = 0;
    for l in f.lines() {
        let l = l.unwrap();
        let t = trim_start(&l, "OPENSSL_CRYPTO_LIBRARY:FILEPATH=")
            .or_else(|| trim_start(&l, "OPENSSL_SSL_LIBRARY:FILEPATH="));
        if let Some(s) = t {
            let path = Path::new(s);
            println!(
                "cargo:rustc-link-search=native={}",
                path.parent().unwrap().display()
            );
            cnt += 1;
        }
    }
    if cnt != 2 {
        panic!(
            "CMake cache invalid, file {} contains {} ssl keys!",
            path, cnt
        );
    }
    println!("cargo:rustc-link-lib=ssl");
    println!("cargo:rustc-link-lib=crypto");
}

fn setup_libz(config: &mut Config) {
    config.define("gRPC_ZLIB_PROVIDER", "package");
    config.register_dep("Z");
    // cmake script expect libz.a being under ${DEP_Z_ROOT}/lib, but libz-sys crate put it
    // under ${DEP_Z_ROOT}/build. Append the path to CMAKE_PREFIX_PATH to get around it.
    let zlib_root = env::var("DEP_Z_ROOT").unwrap();
    let prefix_path = if let Ok(prefix_path) = env::var("CMAKE_PREFIX_PATH") {
        format!("{};{}/build", prefix_path, zlib_root)
    } else {
        format!("{}/build", zlib_root)
    };
    // To avoid linking system library, set lib path explicitly.
    println!("cargo:rustc-link-search=native={}/build", zlib_root);
    println!("cargo:rustc-link-search=native={}/lib", zlib_root);
    env::set_var("CMAKE_PREFIX_PATH", prefix_path);
}

#[cfg(feature = "openssl-vendored")]
fn setup_openssl(config: &mut Config) {
    // openssl-sys uses openssl-src to build the library. openssl-src uses
    // configure/make to build the library which makes it hard to detect
    // what's the actual path of the library. Here assumes the directory
    // structure as follow (which is the behavior of 0.9.47):
    // install_dir/
    //     include/
    //     lib/
    // Remove the hack when sfackler/rust-openssl#1117 is resolved.
    config.register_dep("openssl");
    if env::var("DEP_OPENSSL_ROOT").is_err() {
        let include_str = env::var("DEP_OPENSSL_INCLUDE").unwrap();
        let include_dir = Path::new(&include_str);
        let root_dir = format!("{}", include_dir.parent().unwrap().display());
        env::set_var("DEP_OPENSSL_ROOT", &root_dir);
    }
}

#[cfg(not(feature = "openssl-vendored"))]
fn setup_openssl(_config: &mut Config) {}

fn get_env(name: &str) -> Option<String> {
    println!("cargo:rerun-if-env-changed={}", name);
    match env::var(name) {
        Ok(s) => Some(s),
        Err(VarError::NotPresent) => None,
        Err(VarError::NotUnicode(s)) => {
            panic!("unrecognize env var of {}: {:?}", name, s.to_string_lossy());
        }
    }
}

// Generate the bindings to grpc C-core.
// Try to disable the generation of platform-related bindings.
fn bindgen_grpc(mut config: bindgen::Builder, file_path: &PathBuf) {
    // Search header files with API interface
    let mut headers = Vec::new();
    for result in WalkDir::new(Path::new("./grpc/include")) {
        let dent = result.expect("Error happened when search headers");
        if !dent.file_type().is_file() {
            continue;
        }
        let mut file = fs::File::open(dent.path()).expect("couldn't open headers");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Coundn't read header content");
        if buf.contains("GRPCAPI") || buf.contains("GPRAPI") {
            headers.push(String::from(dent.path().to_str().unwrap()));
        }
    }

    // To control the order of bindings
    headers.sort();
    for path in headers {
        config = config.header(path);
    }

    let cfg = config
        .header("grpc_wrap.cc")
        .clang_arg("-xc++")
        .clang_arg("-I./grpc/include")
        .clang_arg("-std=c++11")
        .rustfmt_bindings(true)
        .impl_debug(true)
        .whitelist_function(r"\bgrpc_.*")
        .whitelist_function(r"\bgpr_.*")
        .whitelist_function(r"\bgrpcwrap_.*")
        .whitelist_var(r"\bGRPC_.*")
        .whitelist_type(r"\bgrpc_.*")
        .whitelist_type(r"\bgpr_.*")
        .whitelist_type(r"\bgrpcwrap_.*")
        .whitelist_type(r"\bcensus_context.*")
        .whitelist_type(r"\bverify_peer_options.*")
        .blacklist_type(r"(__)?pthread.*")
        .blacklist_function(r"\bgpr_mu_.*")
        .blacklist_function(r"\bgpr_cv_.*")
        .blacklist_function(r"\bgpr_once_.*")
        .blacklist_type(r"gpr_mu")
        .blacklist_type(r"gpr_cv")
        .blacklist_type(r"gpr_once")
        .constified_enum_module(r"grpc_status_code")
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        });
    println!("running {}", cfg.command_line_flags().join(" "));
    cfg.generate()
        .expect("Unable to generate grpc bindings")
        .write_to_file(file_path)
        .expect("Couldn't write bindings!");
}

// Determine if need to update bindings. Supported platforms do not
// need to be updated by default unless the UPDATE_BIND is specified.
// Other platforms use bindgen to generate the bindings every time.
fn config_binding_path(config: bindgen::Builder) {
    let file_path: PathBuf;
    println!("cargo:rerun-if-changed=bindings/x86_64-unknown-linux-gnu-bindings.rs");
    match env::var("TARGET").unwrap().as_str() {
        "x86_64-unknown-linux-gnu" => {
            file_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
                .join("bindings")
                .join("x86_64-unknown-linux-gnu-bindings.rs");
            if env::var("UPDATE_BIND").map(|s| s == "1").unwrap_or(false) {
                bindgen_grpc(config, &file_path);
            }
        }
        _ => {
            file_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("grpc-bindings.rs");
            bindgen_grpc(config, &file_path);
        }
    };
    println!(
        "cargo:rustc-env=BINDING_PATH={}",
        file_path.to_str().unwrap()
    );
}

fn main() {
    println!("cargo:rerun-if-changed=grpc_wrap.cc");
    println!("cargo:rerun-if-changed=grpc");
    println!("cargo:rerun-if-env-changed=UPDATE_BIND");

    let mut cc = Build::new();
    let mut bind_config = bindgen::Builder::default();

    let library = if cfg!(feature = "secure") {
        cc.define("GRPC_SYS_SECURE", None);
        bind_config = bind_config.clang_arg("-DGRPC_SYS_SECURE");
        "grpc"
    } else {
        "grpc_unsecure"
    };

    if get_env("CARGO_CFG_TARGET_OS").map_or(false, |s| s == "windows") {
        // At lease win7
        cc.define("_WIN32_WINNT", Some("0x0700"));
        bind_config = bind_config.clang_arg("-D _WIN32_WINNT=0x0700");
    }

    if get_env("GRPCIO_SYS_USE_PKG_CONFIG").map_or(false, |s| s == "1") {
        // Print cargo metadata.
        let lib_core = probe_library(library, true);
        for inc_path in lib_core.include_paths {
            cc.include(inc_path);
        }
    } else {
        build_grpc(&mut cc, library);
    }

    cc.cpp(true);
    if !cfg!(target_env = "msvc") {
        cc.flag("-std=c++11");
    }
    cc.file("grpc_wrap.cc");

    cc.warnings_into_errors(true);

    cc.compile("libgrpc_wrap.a");

    config_binding_path(bind_config);
}
