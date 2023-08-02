// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use std::collections::HashSet;
use std::env::VarError;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

use cmake::Config as CmakeConfig;
use pkg_config::{Config as PkgConfig, Library};
use walkdir::WalkDir;

const GRPC_VERSION: &str = "1.44.0";

include!("link-deps.rs");

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
    let modules = vec![
        "grpc",
        "grpc/third_party/cares/cares",
        "grpc/third_party/address_sorting",
        "grpc/third_party/abseil-cpp",
        "grpc/third_party/re2",
    ];

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

/// If cache is stale, remove it to avoid compilation failure.
fn clean_up_stale_cache(cxx_compiler: String) {
    // We don't know the cmake output path before it's configured.
    let build_dir = format!("{}/build", env::var("OUT_DIR").unwrap());
    let path = format!("{build_dir}/CMakeCache.txt");
    let f = match std::fs::File::open(path) {
        Ok(f) => BufReader::new(f),
        // It may be an empty directory.
        Err(_) => return,
    };
    let cache_stale = f.lines().any(|l| {
        let l = l.unwrap();
        trim_start(&l, "CMAKE_CXX_COMPILER:").map_or(false, |s| {
            let mut splits = s.splitn(2, '=');
            splits.next();
            splits.next().map_or(false, |p| p != cxx_compiler)
        })
    });
    // CMake can't handle compiler change well, it will invalidate cache without respecting command
    // line settings and result in configuration failure.
    // See https://gitlab.kitware.com/cmake/cmake/-/issues/18959.
    if cache_stale {
        let _ = fs::remove_dir_all(&build_dir);
    }
}

/// List packages needed for linking in working directory.
fn list_packages(dst: &Path) {
    env::set_var(
        "PKG_CONFIG_PATH",
        format!("{}/lib/pkgconfig", dst.display()),
    );
    let mut cfg = PkgConfig::new();
    cfg.print_system_cflags(false)
        .print_system_libs(false)
        .env_metadata(false)
        .cargo_metadata(false)
        .atleast_version(GRPC_VERSION);
    let grpc = cfg.probe("grpc").unwrap();
    let mut grpc_libs: HashSet<_> = grpc.libs.iter().cloned().collect();
    let grpc_unsecure = cfg.probe("grpc_unsecure").unwrap();
    let mut grpc_unsecure_libs: HashSet<_> = grpc_unsecure.libs.iter().cloned().collect();

    // grpc_unsecure.pc is not accurate, see also grpc/grpc#24512. Should also include "address_sorting", "upb", "cares", "z".
    const EXTRA_LIBS: [&str; 5] = ["address_sorting", "upb", "cares", "r2", "z"];
    grpc_unsecure_libs.extend(EXTRA_LIBS.iter().map(ToString::to_string));
    grpc_libs.extend(EXTRA_LIBS.iter().map(ToString::to_string));
    // There is no "rt" on Windows and MacOS.
    grpc_libs.remove("rt");
    grpc_unsecure_libs.remove("rt");

    // ssl, crypto is managed by us according to different features.
    grpc_libs.remove("ssl");
    grpc_libs.remove("crypto");

    let mut common_libs: Vec<_> = grpc_libs.intersection(&grpc_unsecure_libs).collect();
    let mut secure_only: Vec<_> = grpc_libs.difference(&grpc_unsecure_libs).collect();
    let mut unsecure_only: Vec<_> = grpc_unsecure_libs.difference(&grpc_libs).collect();

    common_libs.sort();
    secure_only.sort();
    unsecure_only.sort();

    let outputs = &[
        ("COMMON_DEPS", common_libs),
        ("GRPC_DEPS", secure_only),
        ("GRPC_UNSECURE_DEPS", unsecure_only),
    ];

    let mut f = File::create("link-deps.rs").unwrap();
    f.write_all(
        b"/// Following two arrays are generated by running pkg-config manually. We can
/// also choose to run pkg-config at build time, but it will requires pkg-config
/// in path, which is unfriendly for platforms like Windows.
",
    )
    .unwrap();
    for (name, libs) in outputs {
        writeln!(f, "const {name}: &[&str] = &[").unwrap();
        for lib in libs {
            writeln!(f, "\"{lib}\",").unwrap();
        }
        writeln!(f, "];").unwrap();
    }
}

fn build_grpc(cc: &mut cc::Build, library: &str) {
    prepare_grpc();

    let target = env::var("TARGET").unwrap();
    let dst = {
        let mut config = CmakeConfig::new("grpc");

        if get_env("CARGO_CFG_TARGET_OS").map_or(false, |s| s == "macos") {
            config.cxxflag("-stdlib=libc++");
            println!("cargo:rustc-link-lib=resolv");
        }

        // Ensure CoreFoundation be found in macos or ios
        if get_env("CARGO_CFG_TARGET_OS").map_or(false, |s| s == "macos")
            || get_env("CARGO_CFG_TARGET_OS").map_or(false, |s| s == "ios")
        {
            println!("cargo:rustc-link-lib=framework=CoreFoundation");
        }

        let cxx_compiler = if let Some(val) = get_env("CXX") {
            config.define("CMAKE_CXX_COMPILER", val.clone());
            val
        } else if env::var("CARGO_CFG_TARGET_ENV").unwrap() == "musl" {
            config.define("CMAKE_CXX_COMPILER", "g++");
            "g++".to_owned()
        } else {
            format!("{}", cc.get_compiler().path().display())
        };
        clean_up_stale_cache(cxx_compiler);

        // Cross-compile support for iOS
        match target.as_str() {
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
        config.define("gRPC_INSTALL", cfg!(feature = "_list-package").to_string());
        // We don't need to build csharp target.
        config.define("gRPC_BUILD_CSHARP_EXT", "false");
        // We don't need to build codegen target.
        config.define("gRPC_BUILD_CODEGEN", "false");
        // We don't need to build benchmarks.
        config.define("gRPC_BENCHMARK_PROVIDER", "none");
        // Check https://github.com/protocolbuffers/protobuf/issues/12185
        config.define("ABSL_ENABLE_INSTALL", "ON");
        if cfg!(feature = "internals") {
            config.define("ABSL_PROPAGATE_CXX_STD", "ON");
        }

        // `package` should only be set for secure feature, otherwise cmake will always search for
        // ssl library.
        if cfg!(feature = "_secure") {
            config.define("gRPC_SSL_PROVIDER", "package");
        }
        #[cfg(feature = "_secure")]
        if cfg!(feature = "openssl") {
            if cfg!(feature = "openssl-vendored") {
                config.register_dep("openssl");
            }
        } else {
            #[cfg(feature = "boringssl")]
            build_boringssl(&mut config);
        }
        if cfg!(feature = "no-omit-frame-pointer") {
            config
                .cflag("-fno-omit-frame-pointer")
                .cxxflag("-fno-omit-frame-pointer");
        }
        // Uses zlib from libz-sys.
        setup_libz(&mut config);
        if !cfg!(feature = "_list-package") {
            config.build_target(library);
        }
        config.uses_cxx11().build()
    };

    let lib_suffix = if target.contains("msvc") {
        ".lib"
    } else {
        ".a"
    };
    let build_dir = format!("{}/build", dst.display());
    for e in WalkDir::new(&build_dir) {
        let e = e.unwrap();
        if e.file_name().to_string_lossy().ends_with(lib_suffix) {
            println!(
                "cargo:rustc-link-search=native={}",
                e.path().parent().unwrap().display()
            );
        }
    }

    if cfg!(feature = "_list-package") {
        list_packages(&dst);
    }

    let libs = if library.contains("unsecure") {
        GRPC_UNSECURE_DEPS
    } else {
        GRPC_DEPS
    };
    for l in COMMON_DEPS.iter().chain(libs) {
        println!("cargo:rustc-link-lib=static={l}");
    }

    if cfg!(feature = "_secure") {
        if cfg!(feature = "openssl") && !cfg!(feature = "openssl-vendored") {
            figure_ssl_path(&build_dir);
        } else {
            println!("cargo:rustc-link-lib=static=ssl");
            println!("cargo:rustc-link-lib=static=crypto");
        }
    }

    cc.include("grpc/include");
    if cfg!(feature = "internals") {
        cc.include("grpc");
        cc.include("grpc/third_party/abseil-cpp");
    }
}

fn figure_ssl_path(build_dir: &str) {
    let path = format!("{build_dir}/CMakeCache.txt");
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

#[cfg(feature = "boringssl")]
fn build_boringssl(config: &mut CmakeConfig) {
    let boringssl_artifact = boringssl_src::Build::new().build();
    config.define(
        "OPENSSL_ROOT_DIR",
        format!("{}", boringssl_artifact.root_dir().display()),
    );
    // To avoid linking system library, set lib path explicitly.
    println!(
        "cargo:rustc-link-search=native={}",
        boringssl_artifact.lib_dir().display()
    );
}

fn setup_libz(config: &mut CmakeConfig) {
    config.define("gRPC_ZLIB_PROVIDER", "package");
    config.register_dep("Z");
    // cmake script expect libz.a being under ${DEP_Z_ROOT}/lib, but libz-sys crate put it
    // under ${DEP_Z_ROOT}/build. Append the path to CMAKE_PREFIX_PATH to get around it.
    let zlib_root = env::var("DEP_Z_ROOT").unwrap();
    let prefix_path = if let Ok(prefix_path) = env::var("CMAKE_PREFIX_PATH") {
        format!("{prefix_path};{zlib_root}/build")
    } else {
        format!("{zlib_root}/build")
    };
    // To avoid linking system library, set lib path explicitly.
    println!("cargo:rustc-link-search=native={zlib_root}/build");
    println!("cargo:rustc-link-search=native={zlib_root}/lib");
    env::set_var("CMAKE_PREFIX_PATH", prefix_path);
}

fn get_env(name: &str) -> Option<String> {
    println!("cargo:rerun-if-env-changed={name}");
    match env::var(name) {
        Ok(s) => Some(s),
        Err(VarError::NotPresent) => None,
        Err(VarError::NotUnicode(s)) => {
            panic!("unrecognize env var of {name}: {:?}", s.to_string_lossy());
        }
    }
}

enum Binding {
    GrpcWrap {
        #[allow(dead_code)]
        src: &'static str,
        dest: &'static str,
        env: &'static str,
    },
    GrpcWrapStats {
        #[allow(dead_code)]
        src: &'static str,
        dest: &'static str,
        env: &'static str,
    },
}

impl Binding {
    fn env(&self) -> &str {
        match self {
            Self::GrpcWrap { env, .. } | Self::GrpcWrapStats { env, .. } => env,
        }
    }
    fn dest(&self) -> &str {
        match self {
            Self::GrpcWrap { dest, .. } | Self::GrpcWrapStats { dest, .. } => dest,
        }
    }
}

// Generate the bindings to grpc C-core.
// Try to disable the generation of platform-related bindings.
#[cfg(any(
    feature = "_gen-bindings",
    not(all(
        any(target_os = "linux", target_os = "macos"),
        any(target_arch = "x86_64", target_arch = "aarch64")
    ))
))]
fn bindgen_grpc(binding: Binding, dest_path: &Path) {
    // create a config to generate binding file
    let mut config = bindgen::Builder::default();
    if cfg!(feature = "_secure") {
        config = config.clang_arg("-DGRPC_SYS_SECURE");
    }

    if get_env("CARGO_CFG_TARGET_OS").map_or(false, |s| s == "windows") {
        config = config.clang_arg("-D _WIN32_WINNT=0x600");
    }

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

    println!("cargo:rerun-if-env-changed=TEST_BIND");
    let gen_tests = env::var("TEST_BIND").map_or(false, |s| s == "1");

    let mut cfg = config
        .clang_arg("-xc++")
        .clang_arg("-I./grpc/include")
        .clang_arg("-std=c++14")
        .rustfmt_bindings(true)
        .impl_debug(true)
        .size_t_is_usize(true)
        .disable_header_comment()
        .allowlist_function(r"\bgrpcwrap_.*")
        .allowlist_type(r"\bgrpcwrap_.*")
        // Block all system headers.
        .blocklist_file(r"^/.*")
        .blocklist_function(r"\bgpr_mu_.*")
        .blocklist_function(r"\bgpr_cv_.*")
        .blocklist_function(r"\bgpr_once_.*")
        .blocklist_type(r"gpr_mu")
        .blocklist_type(r"gpr_cv")
        .blocklist_type(r"gpr_once")
        .constified_enum_module(r"grpc_status_code")
        .layout_tests(gen_tests)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        });
    match binding {
        // Generate grpc_wrap.cc bindings.
        Binding::GrpcWrap { src, .. } => {
            cfg = cfg
                .header(src)
                .allowlist_function(r"\bgrpc_.*")
                .allowlist_function(r"\bgpr_.*")
                .allowlist_var(r"\bGRPC_.*")
                .allowlist_type(r"\bgrpc_.*")
                .allowlist_type(r"\bgpr_.*")
                .allowlist_type(r"\bcensus_context.*")
                .allowlist_type(r"\bverify_peer_options.*");
        }
        // Generate grpc_wrap_stats.cc bindings.
        Binding::GrpcWrapStats { src, .. } => {
            cfg = cfg
                .header(src)
                .clang_arg("-I./grpc")
                .clang_arg("-I./grpc/third_party/abseil-cpp")
                .blocklist_function(r"\bgrpc_.*")
                .blocklist_type(r"\bgrpc_.*");
        }
    }
    println!("running {}", cfg.command_line_flags().join(" "));
    cfg.generate()
        .expect("Unable to generate grpc bindings")
        .write_to_file(dest_path)
        .expect("Couldn't write bindings!");
}

// Determine if need to update bindings. Supported platforms do not
// need to be updated by default unless the _gen-bindings feature is specified.
// Other platforms use bindgen to generate the bindings every time.
fn config_binding_path() {
    let config_binding = |binding: Binding| {
        let target = env::var("TARGET").unwrap();
        let dest_path = match target.as_str() {
            "x86_64-unknown-linux-gnu"
            | "x86_64-unknown-linux-musl"
            | "aarch64-unknown-linux-musl"
            | "aarch64-unknown-linux-gnu"
            | "x86_64-apple-darwin"
            | "aarch64-apple-darwin" => {
                // Cargo treats nonexistent files changed, so we only emit the rerun-if-changed
                // directive when we expect the target-specific pre-generated binding file to be
                // present.
                println!("cargo:rerun-if-changed=bindings/{}", binding.dest());

                PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
                    .join("bindings")
                    .join(binding.dest())
            }
            _ => {
                PathBuf::from(env::var("OUT_DIR").unwrap()).join(format!("grpc-{}", binding.dest()))
            }
        };
        println!(
            "cargo:rustc-env={}={}",
            binding.env(),
            dest_path.to_str().unwrap()
        );
        #[cfg(any(
            feature = "_gen-bindings",
            not(all(
                any(target_os = "linux", target_os = "macos"),
                any(target_arch = "x86_64", target_arch = "aarch64")
            ))
        ))]
        {
            // On some system (like Windows), stack size of main thread may
            // be too small.
            let f = dest_path.clone();
            std::thread::Builder::new()
                .stack_size(8 * 1024 * 1024)
                .name("bindgen_grpc".to_string())
                .spawn(move || {
                    bindgen_grpc(binding, &f);
                })
                .unwrap()
                .join()
                .unwrap();
        }
        let _ = binding;
    };

    config_binding(Binding::GrpcWrap {
        src: "grpc_wrap.cc",
        dest: "bindings.rs",
        env: "BINDING_WRAP_PATH",
    });
    if cfg!(feature = "internals") {
        config_binding(Binding::GrpcWrapStats {
            src: "grpc_wrap_stats.cc",
            dest: "bindings_stats.rs",
            env: "BINDING_WRAP_STATS_PATH",
        });
    }
}

fn main() {
    println!("cargo:rerun-if-changed=grpc_wrap.cc");
    println!("cargo:rerun-if-changed=grpc_wrap_stats.cc");
    println!("cargo:rerun-if-changed=grpc");

    // create a builder to compile grpc_wrap.cc
    let mut cc = cc::Build::new();

    let library = if cfg!(feature = "_secure") {
        cc.define("GRPC_SYS_SECURE", None);
        "grpc"
    } else {
        "grpc_unsecure"
    };

    if get_env("CARGO_CFG_TARGET_OS").map_or(false, |s| s == "windows") {
        // At lease vista
        cc.define("_WIN32_WINNT", Some("0x600"));
    }

    if !cfg!(feature = "internals")
        && get_env("GRPCIO_SYS_USE_PKG_CONFIG").map_or(false, |s| s == "1")
    {
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
        cc.flag("-std=c++14");
    }
    cc.file("grpc_wrap.cc");
    if cfg!(feature = "internals") {
        cc.file("grpc_wrap_stats.cc");
    }
    cc.warnings_into_errors(true);
    cc.compile("libgrpc_wrap.a");

    config_binding_path();
}
