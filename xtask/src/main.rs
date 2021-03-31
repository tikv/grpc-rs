use std::fs::{self, File};
use std::process::{self, Command};
use std::{
    env,
    io::{Read, Write},
    str,
};

fn print_help() {
    eprintln!("cargo xtask [subcommand]");
    eprintln!();
    eprintln!("Supported subcommands are:");
    eprintln!("\tbindgen\tGenerate rust-bindgen for grpcio-sys package");
    eprintln!("\tsubmodule\tInit necessary submodules for compilation");
    eprintln!("\tclang-lint\tLint cpp code in grpcio-sys package");
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

fn bindgen() {
    let arch = match env::var("ARCH") {
        Ok(arch) => arch,
        Err(_) => str::from_utf8(&Command::new("uname").arg("-p").output().unwrap().stdout)
            .unwrap()
            .trim()
            .to_string(),
    };
    let tuple = format!("{}-unknown-linux-gnu", arch);
    exec(
        cargo()
            .env("UPDATE_BIND", "1")
            .args(&["build", "-p", "grpcio-sys", "--target", &tuple]),
    );
    for f in fs::read_dir("grpc-sys/bindings").unwrap() {
        let p = f.unwrap().path();
        exec(Command::new("rustfmt").arg(format!("{}", p.display())));
        let mut content = String::new();
        File::open(&p)
            .unwrap()
            .read_to_string(&mut content)
            .unwrap();
        let mut modified = false;
        let s: String = content
            .lines()
            .filter(|l| {
                if !l.starts_with("pub type ") || !l.contains("= ::std::os::raw::") {
                    true
                } else {
                    modified = true;
                    false
                }
            })
            .collect();
        if modified {
            File::create(&p).unwrap().write_all(s.as_bytes()).unwrap();
        }
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

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        print_help();
    }
    args.next();
    let subcommand = args.next().unwrap();
    match &*subcommand {
        "bindgen" => bindgen(),
        "submodule" => submodule(),
        "clang-lint" => clang_lint(),
        _ => print_help(),
    }
}
