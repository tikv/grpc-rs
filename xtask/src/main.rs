use std::process::{self, Command};
use std::{
    env,
    io::{Read, Write},
    str,
};
use std::{
    fs::{self, File},
    path::Path,
};

fn print_help() {
    eprintln!("cargo xtask [subcommand]");
    eprintln!();
    eprintln!("Supported subcommands are:");
    eprintln!("\tbindgen\tGenerate rust-bindgen for grpcio-sys package");
    eprintln!("\tsubmodule\tInit necessary submodules for compilation");
    eprintln!("\tclang-lint\tLint cpp code in grpcio-sys package");
    eprintln!("\tcodegen\tGenerate rust code for all protocols");
}

fn cargo() -> Command {
    match env::var("CARGO") {
        Ok(s) => Command::new(s),
        Err(_) => {
            eprintln!("no CARGO in environment variables, please invoke the binary by cargo xtask");
            process::exit(1);
        }
    }
}

fn exec(c: &mut Command) {
    if let Err(e) = c.status() {
        eprintln!("failed to execute {:?}: {}", c, e);
        process::exit(-1);
    }
}

fn find_default_arch() -> String {
    let s = String::from_utf8(
        Command::new("rustc")
            .args(&["--print", "cfg"])
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();
    for l in s.lines() {
        if let Some(arch) = l.strip_prefix("target_arch=") {
            if !arch.is_empty() {
                return arch[1..arch.len() - 1].to_string();
            }
        }
    }
    panic!("arch not found in {:?}", s);
}

fn remove_match(data: &str, pattern: impl Fn(&str) -> bool) -> String {
    let mut target = String::with_capacity(data.len());
    for l in data.lines() {
        if pattern(l) {
            continue;
        }
        target.push_str(l);
        target.push('\n');
    }
    target
}

fn bindgen() {
    let arch = match env::var("ARCH") {
        Ok(arch) => arch,
        Err(_) => find_default_arch(),
    };
    let tuple = format!("{}-unknown-linux-gnu", arch);
    exec(
        cargo()
            .env("UPDATE_BIND", "1")
            .args(&["build", "-p", "grpcio-sys", "--target", &tuple]),
    );
    for f in fs::read_dir("grpc-sys/bindings").unwrap() {
        let p = f.unwrap().path();
        let mut content = String::new();
        File::open(&p)
            .unwrap()
            .read_to_string(&mut content)
            .unwrap();
        let content = remove_match(&content, |l| {
            l.starts_with("pub type ") && l.contains("= ::std::os::raw::")
        });
        File::create(&p)
            .unwrap()
            .write_all(content.as_bytes())
            .unwrap();
    }
}

fn cmd(c: &str) -> Command {
    Command::new(c)
}

fn cmd_in(c: &str, dir: &str) -> Command {
    let mut cmd = cmd(c);
    cmd.current_dir(dir);
    cmd
}

fn submodule() {
    exec(cmd("git").args(&["submodule", "update", "--init", "grpc-sys/grpc"]));
    for dir in &["cares/cares", "abseil-cpp", "re2"] {
        exec(cmd_in("git", "grpc-sys/grpc/third_party").args(&[
            "submodule",
            "update",
            "--init",
            &dir,
        ]));
    }
    for e in fs::read_dir("grpc-sys/grpc/third_party/boringssl-with-bazel/").unwrap() {
        let e = e.unwrap();
        if e.file_type().unwrap().is_dir() {
            fs::remove_dir_all(e.path()).unwrap();
        } else {
            fs::remove_file(e.path()).unwrap();
        }
    }
    exec(cmd_in("git", "grpc-sys/grpc/third_party/zlib").args(&["clean", "-df"]));
    exec(cmd_in("git", "grpc-sys/grpc/third_party/zlib").args(&["reset", "--hard"]));
}

fn clang_lint() {
    exec(cmd("clang-tidy").args(&[
        "grpc-sys/grpc_wrap.cc",
        "--",
        "-Igrpc-sys/grpc/include",
        "-x",
        "c++",
        "-std=c++11",
    ]));
    exec(cmd("clang-format").args(&["-i", "grpc-sys/grpc_wrap.cc"]));
}

const PROTOS: &[(&str, &[&str], &str)] = &[(
    "grpc-sys/grpc/src/proto",
    &["grpc/health/v1"],
    "health/src/proto",
)];

const NAMING_PATCH: &[(&str, &[(&str, &str)])] = &[(
    "health/src/proto/protobuf/health.rs",
    &[
        ("HealthCheckResponse_ServingStatus", "ServingStatus"),
        // Order is important.
        ("NOT_SERVING", "NotServing"),
        ("SERVICE_UNKNOWN", "ServiceUnknown"),
        ("UNKNOWN", "Unknown"),
        ("SERVING", "Serving"),
        ("rustfmt_skip", "rustfmt::skip"),
    ],
)];

fn generate_protobuf(protoc: &str, include: &str, inputs: &[&str], out_dir: &str) {
    if Path::new(out_dir).exists() {
        fs::remove_dir_all(out_dir).unwrap();
    }
    fs::create_dir_all(out_dir).unwrap();

    // TODO: update rust-protobuf to allow specifying protoc explicitly.
    protoc_rust::run(protoc_rust::Args {
        out_dir,
        includes: &[include],
        input: inputs,
        customize: protoc_rust::Customize::default(),
    })
    .unwrap();

    exec(cargo().args(&["build", "-p", "grpcio-compiler"]));
    let mut c = cmd(&protoc);
    c.arg(format!("-I{}", include))
        .arg(format!("--grpc_out={}", out_dir))
        .arg("--plugin=protoc-gen-grpc=./target/debug/grpc_rust_plugin");
    for i in inputs {
        c.arg(i);
    }
    exec(&mut c);

    for (path, name_fixes) in NAMING_PATCH {
        let mut content = String::new();
        File::open(path)
            .unwrap()
            .read_to_string(&mut content)
            .unwrap();
        for (src, target) in *name_fixes {
            content = content.replace(src, target);
        }
        content = remove_match(&content, |l| l.contains("::protobuf::VERSION"));
        File::create(path)
            .unwrap()
            .write_all(content.as_bytes())
            .unwrap();
    }
}

fn generate_prost(protoc: &str, include: &str, inputs: &[&str], out_dir: &str) {
    env::set_var("PROTOC", protoc);
    if Path::new(out_dir).exists() {
        fs::remove_dir_all(out_dir).unwrap();
    }
    fs::create_dir_all(out_dir).unwrap();
    exec(
        cargo()
            .args(&[
                "build",
                "--no-default-features",
                "--features",
                "prost-codec",
                "--bin",
                "grpc_rust_prost",
            ])
            .current_dir("compiler"),
    );
    exec(
        Command::new("target/debug/grpc_rust_prost")
            .arg(format!("--protos={}", inputs.join(",")))
            .arg(format!("--includes={}", include))
            .arg(format!("--out-dir={}", out_dir)),
    );
}

fn codegen() {
    let protoc = if cmd("protoc").arg("--version").output().is_ok() {
        // Prefer M1 version of protoc.
        "protoc".to_string()
    } else {
        prost_build::protoc()
            .into_os_string()
            .to_str()
            .unwrap()
            .to_string()
    };
    for (include, protos, out_dir) in PROTOS {
        let inputs: Vec<_> = protos
            .iter()
            .flat_map(|p| {
                fs::read_dir(format!("{}/{}", include, p))
                    .unwrap()
                    .filter_map(|e| {
                        let e = e.unwrap();
                        match e.path().extension() {
                            Some(s) if s == "proto" => Some(format!("{}", e.path().display())),
                            _ => None,
                        }
                    })
            })
            .collect();
        let inputs_ref: Vec<_> = inputs.iter().map(|s| s.as_str()).collect();
        generate_protobuf(
            &protoc,
            include,
            &inputs_ref,
            &format!("{}/protobuf", out_dir),
        );
        generate_prost(&protoc, include, &inputs_ref, &format!("{}/prost", out_dir));
    }
    exec(cargo().args(&["fmt", "--all"]))
}

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        print_help();
        process::exit(1);
    }
    args.next();
    let subcommand = args.next().unwrap();
    match &*subcommand {
        "bindgen" => bindgen(),
        "submodule" => submodule(),
        "clang-lint" => clang_lint(),
        "codegen" => codegen(),
        _ => print_help(),
    }
}
