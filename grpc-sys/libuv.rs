use std::fs;
use std::path::{Path, PathBuf};
use std::env;
use std::process::Command;

pub fn libuv_dir() -> PathBuf {
    Path::new("libuv").to_owned()
}
fn libuv_repo() -> String {
    "https://github.com/libuv/libuv.git".to_owned()
}
fn libuv_revision() -> String {
    "v1.9.1".to_owned()
}
fn libuv_lib() -> PathBuf {
    if cfg!(windows) {
        libuv_dir().join("Release").join("lib").join("libuv.lib")
    } else {
        libuv_dir().join(".libs").join("libuv.a")
    }
}

fn libuv_force_fetch() -> bool {
    env::var("LIBUV_SYS_FORCE_FETCH").is_ok()
}
fn libuv_clean_compile() -> bool {
    env::var("LIBUV_SYS_CLEAN_COMPILE").is_ok()
}

fn download_libuv() {
    println!("Downloading libuv...");
    fs::create_dir_all(libuv_dir()).unwrap();
    Command::new("git")
        .arg("clone")
        .arg("-b")
        .arg(libuv_revision())
        .arg(libuv_repo())
        .arg(libuv_dir())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    fs::create_dir_all("libuv.stamp").unwrap();
}

fn build_libuv() {
    println!("Building libuv...");
    // don't support cross-compiling ...
    if cfg!(windows) {
        let libuv_arch = if cfg!(target_arch = "x86") {
            "x86"
        } else if cfg!(target_arch = "x86_64") {
            "x64"
        } else {
            panic!("Unsupported Windows host architecture")
        };
        // deleted some environment stuff ("setup_windows_env") that looked fairly
        // Python-specific
        Command::new("cmd.exe")
            .arg("/C")
            .arg("vcbuild.bat")
            .arg(libuv_arch)
            .arg("release")
            .current_dir(libuv_dir())
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    } else {
        Command::new("sh")
            .arg("autogen.sh")
            .current_dir(libuv_dir())
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
        Command::new("./configure")
            .arg("--with-pic")
            .current_dir(libuv_dir())
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
        Command::new("make")
            .current_dir(libuv_dir())
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }
}

fn get_libuv() {
    if libuv_force_fetch() {
        fs::remove_dir_all(libuv_dir()).unwrap();
        fs::remove_dir_all("libuv.stamp").unwrap();
    }

    if fs::metadata("libuv.stamp").is_err() {
        download_libuv();
        build_libuv();
    } else {
        if libuv_clean_compile() {
            if cfg!(windows) {
                Command::new("cmd.exe")
                    .arg("/C")
                    .arg("vcbuild.bat")
                    .arg("clean")
                    .current_dir(libuv_dir())
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();
                fs::remove_dir_all(libuv_dir().join("Release")).unwrap();
            } else {
                Command::new("make")
                    .arg("clean")
                    .current_dir(libuv_dir())
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();
                Command::new("make")
                    .arg("distclean")
                    .current_dir(libuv_dir())
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();
            }
        }

        if fs::metadata(libuv_lib()).is_err() {
            println!("libuv needs to be compiled.");
            build_libuv();
        } else {
            println!("No need to build libuv.");
        }
    }
}

pub fn compile() -> Option<String> {
    let target = env::var("TARGET").unwrap();
    if cfg!(windows) != target.contains("windows") {
        panic!("Cannot cross compile to/from Windows due to use of a batch file build");
    }

    get_libuv();
    println!("cargo:rustc-link-lib=static=uv");
    println!("cargo:rustc-link-search=native={}",
             env::current_dir().unwrap().join(libuv_lib().parent().unwrap()).to_str().unwrap());
    if target.contains("linux") {
        println!("cargo:rustc-link-lib=rt");
    } else if target.contains("windows") {
        println!("cargo:rustc-link-lib=advapi32");
        println!("cargo:rustc-link-lib=iphlpapi");
        println!("cargo:rustc-link-lib=psapi");
        println!("cargo:rustc-link-lib=shell32");
        println!("cargo:rustc-link-lib=userenv");
        println!("cargo:rustc-link-lib=ws2_32");
    } else if target.contains("freebsd") {
        println!("cargo:rustc-link-lib=kvm");
    }
    Some(env::current_dir().unwrap().join(libuv_dir()).to_str().unwrap().to_owned())
}