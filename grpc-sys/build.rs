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
    use super::GRPC_VERSION;

    pub fn build_or_link_grpc(cc: &mut ::gcc::Config) {
        if let Ok(lib) = ::pkg_config::Config::new()
               .atleast_version(GRPC_VERSION)
               .probe("grpc_unsecure") {
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
    use std::fs::DirBuilder;
    use std::process::Command;

    use super::GRPC_VERSION;

    const ZLIB_VERSION: &'static str = "1.2.8";
    const BORINGSSL_COMMIT_HASH: &'static str = "78684e5b222645828ca302e56b40b9daff2b2d27";
    const CARES_TAG: &'static str = "cares-1_12_0";

    struct FakeGitSubmodule {
        url: String,
        base_name: String,
        to_dir: String,
    }

    impl FakeGitSubmodule {
        fn new_with_hash(account: &str,
                         repo: &str,
                         commit_hash: &str,
                         to_dir: &str)
                         -> FakeGitSubmodule {
            FakeGitSubmodule {
                url: format!("https://github.com/{}/{}/archive/{}.tar.gz",
                             account,
                             repo,
                             commit_hash),
                base_name: format!("{}-{}", repo, commit_hash),
                to_dir: to_dir.to_owned(),
            }
        }

        fn new_with_tag(account: &str, repo: &str, tag: &str, to_dir: &str) -> FakeGitSubmodule {
            FakeGitSubmodule {
                url: format!("https://github.com/{}/{}/archive/{}.tar.gz",
                             account,
                             repo,
                             tag),
                base_name: format!("{}-{}", repo, tag),
                to_dir: to_dir.to_owned(),
            }
        }

        fn new_with_semver(account: &str, repo: &str, ver: &str, to_dir: &str) -> FakeGitSubmodule {
            FakeGitSubmodule {
                url: format!("https://github.com/{}/{}/archive/v{}.tar.gz",
                             account,
                             repo,
                             ver),
                base_name: format!("{}-{}", repo, ver),
                to_dir: to_dir.to_owned(),
            }
        }

        fn init(&self) -> Result<(), String> {
            let out_dir = env::var_os("OUT_DIR").unwrap();
            let tgz_file_name = format!("{}.tar.gz", self.base_name);
            try!(Command::new("wget")
                     .args(&["-q", "-c", "-O", &tgz_file_name, &self.url])
                     .current_dir(&out_dir)
                     .status()
                     .map_err(|err| format!("failed to execute wget: {}", err))
                     .and_then(|status| if status.success() {
                                   Ok(())
                               } else {
                                   Err(format!("wget exit with {}", status))
                               }));

            try!(Command::new("tar")
                     .args(&["zxf", &tgz_file_name])
                     .current_dir(&out_dir)
                     .status()
                     .map_err(|err| format!("failed to execute tar: {}", err))
                     .and_then(|status| if status.success() {
                                   Ok(())
                               } else {
                                   Err(format!("tar exit with {}", status))
                               }));
            // clean base dir
            try!(Command::new("rm")
                     .args(&["-r", &self.to_dir])
                     .current_dir(&out_dir)
                     .status()
                     .map_err(|err| format!("failed to execute tar: {}", err)));

            Command::new("mv")
                .args(&[&self.base_name, &self.to_dir])
                .current_dir(&out_dir)
                .status()
                .map_err(|err| format!("mv execute failed: {}", err))
                .and_then(|status| if status.success() {
                              Ok(())
                          } else {
                              Err(format!("mv exit with {}", status))
                          })
        }
    }


    pub fn build_or_link_grpc(cc: &mut ::gcc::Config) {
        let out_dir = env::var("OUT_DIR").expect("Can't access OUT_DIR");
        DirBuilder::new()
            .recursive(true)
            .create(format!("{}/{}", out_dir, "grpc"))
            .unwrap();

        FakeGitSubmodule::new_with_semver("grpc", "grpc", GRPC_VERSION, "grpc")
            .init()
            .unwrap();

        FakeGitSubmodule::new_with_semver("madler", "zlib", ZLIB_VERSION, "grpc/third_party/zlib")
            .init()
            .unwrap();

        FakeGitSubmodule::new_with_hash("google",
                                        "boringssl",
                                        BORINGSSL_COMMIT_HASH,
                                        "grpc/third_party/boringssl")
                .init()
                .unwrap();

        FakeGitSubmodule::new_with_tag("c-ares",
                                       "c-ares",
                                       CARES_TAG,
                                       "grpc/third_party/cares/cares")
                .init()
                .unwrap();

        // fix multiple _main symbols
        let _ = Command::new("rm")
            .current_dir(&format!("{}/grpc/third_party/cares/cares", out_dir))
            .args(&["acountry.c", "adig.c", "ahost.c"])
            .status();

        let dst = ::cmake::Config::new(format!("{}/grpc", out_dir))
            .build_target("grpc")
            .build();

        println!("cargo:rustc-link-search=native={}/build", dst.display());
        println!("cargo:rustc-link-search=native={}/build/third_party/cares",
                 dst.display());
        println!("cargo:rustc-link-search=native={}/build/third_party/zlib",
                 dst.display());
        println!("cargo:rustc-link-search=native={}/build", dst.display());

        println!("cargo:rustc-link-lib=static=z");
        println!("cargo:rustc-link-lib=static=cares");
        println!("cargo:rustc-link-lib=static=gpr");
        println!("cargo:rustc-link-lib=static=grpc");

        cc.include(format!("{}/grpc/include", out_dir));
    }
}

fn main() {
    let mut cc = ::gcc::Config::new();

    imp::build_or_link_grpc(&mut cc);

    cc.file("grpc_wrap.c")
        .flag("-fPIC")
        .flag("-O2")
        .compile("libgrpc_wrap.a");
}
