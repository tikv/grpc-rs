// Copyright 2017 PingCAP, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate gcc;
extern crate cmake;
extern crate pkg_config;

const GRPC_VERSION: &'static str = "1.3.0";

#[cfg(not(feature = "static-link"))]
mod imp {
    use gcc::Config as GccConfig;
    use pkg_config::Config as PkgConfig;

    use super::GRPC_VERSION;

    pub fn build_or_link_grpc(cc: &mut GccConfig) {
        if let Ok(lib) = PkgConfig::new().atleast_version(GRPC_VERSION).probe("grpc") {
            for inc_path in lib.include_paths {
                cc.include(inc_path);
            }
        } else {
            panic!("can't find a dynamic grpc library");
        }
    }
}

#[cfg(feature = "static-link")]
mod imp {
    use std::env;
    use std::process::Command;

    use cmake::Config as CMakeConfig;
    use gcc::Config as GccConfig;

    use super::GRPC_VERSION;

    const ZLIB_VERSION: &'static str = "v1.2.8";
    const BORINGSSL_COMMIT_HASH: &'static str = "78684e5b222645828ca302e56b40b9daff2b2d27";
    const CARES_TAG: &'static str = "cares-1_12_0";

    fn execute(cmd: &mut Command) {
        match cmd.status() {
            Err(e) => panic!("failed to execute {:?}: {}", cmd, e),
            Ok(status) => {
                if !status.success() {
                    panic!("command {:?} exit with {}", cmd, status);
                }
            }
        }
    }

    fn fetch_and_extract(account: &str, repo: &str, arch_tag: &str, to_dir: &str) {
        let url = format!("https://github.com/{}/{}/archive/{}.tar.gz",
                          account,
                          repo,
                          arch_tag);
        let out_dir = env::var("OUT_DIR").unwrap();
        let tgz_file_name = format!("{}-{}.tar.gz", repo, arch_tag);
        let cmds = vec![
            vec!["wget", "-q", "-O", &tgz_file_name, &url],
            vec!["rm", "-rf", &to_dir],
            vec!["mkdir", "-p", &to_dir],
            vec!["tar", "zxf", &tgz_file_name, "-C", to_dir, "--strip-components", "1"],
        ];
        for cmd in cmds {
            execute(Command::new(cmd[0]).args(&cmd[1..]).current_dir(&out_dir));
        }
    }

    fn prepare_grpc() {
        fetch_and_extract("grpc", "grpc", &format!("v{}", GRPC_VERSION), "grpc");

        let submodules =
            vec![
                ("madler", "zlib", ZLIB_VERSION, "grpc/third_party/zlib"),
                ("google", "boringssl", BORINGSSL_COMMIT_HASH, "grpc/third_party/boringssl"),
                ("c-ares", "c-ares", CARES_TAG, "grpc/third_party/cares/cares"),
            ];

        for (account, repo, tag, to_dir) in submodules {
            fetch_and_extract(account, repo, tag, to_dir);
        }
    }

    pub fn build_or_link_grpc(cc: &mut GccConfig) {
        let out_dir = env::var("OUT_DIR").expect("Can't access OUT_DIR");

        prepare_grpc();

        // fix multiple _main symbols
        execute(Command::new("rm")
                    .current_dir(&format!("{}/grpc/third_party/cares/cares", out_dir))
                    .args(&["acountry.c", "adig.c", "ahost.c"]));

        let dst = CMakeConfig::new(format!("{}/grpc", out_dir))
            .build_target("grpc")
            .build();

        let build_dir = format!("{}/build", dst.display());
        println!("cargo:rustc-link-search=native={}", build_dir);
        println!("cargo:rustc-link-search=native={}/third_party/cares",
                 build_dir);
        println!("cargo:rustc-link-search=native={}/third_party/zlib",
                 build_dir);
        println!("cargo:rustc-link-search=native={}/third_party/boringssl/ssl",
                 build_dir);
        println!("cargo:rustc-link-search=native={}/third_party/boringssl/crypto",
                 build_dir);

        println!("cargo:rustc-link-lib=static=z");
        println!("cargo:rustc-link-lib=static=cares");
        println!("cargo:rustc-link-lib=static=gpr");
        println!("cargo:rustc-link-lib=static=grpc");
        println!("cargo:rustc-link-lib=static=ssl");
        println!("cargo:rustc-link-lib=static=crypto");

        cc.include(format!("{}/grpc/include", out_dir));
    }
}

fn main() {
    let mut cc = gcc::Config::new();

    imp::build_or_link_grpc(&mut cc);

    cc.file("grpc_wrap.c")
        .flag("-fPIC")
        .flag("-O2")
        .compile("libgrpc_wrap.a");
}
