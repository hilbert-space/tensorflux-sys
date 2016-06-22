extern crate pkg_config;

use std::path::{Path, PathBuf};
use std::process::Command;

const SOURCE_URL: &'static str = "https://github.com/tensorflow/tensorflow.git";

macro_rules! get(($name:expr) => (::std::env::var($name).unwrap()));

fn main() {
    if pkg_config::find_library("tensorflow").is_ok() {
        return;
    }

    let source = PathBuf::from(&get!("CARGO_MANIFEST_DIR")).join("source");
    let output = PathBuf::from(&get!("OUT_DIR"));

    if !Path::new(&source.join(".git")).exists() {
        run("git", |command| command.arg("clone")
                                    .arg("--branch=v0.9.0rc0")
                                    .arg("--recurse-submodules")
                                    .arg(SOURCE_URL)
                                    .arg(&source));
    }

    run("./configure", |command| command.current_dir(&source));

    run("bazel", |command| command.current_dir(&source)
                                  .arg(format!("--output_base={}", output.display()))
                                  .arg("build")
                                  .arg(format!("--jobs={}", get!("NUM_JOBS")))
                                  .arg("--compilation_mode=opt")
                                  .arg("tensorflow:libtensorflow.so"));

    println!("cargo:rustc-link-lib=dylib=tensorflow");
    println!("cargo:rustc-link-search={}", find(&output, "libtensorflow.so").unwrap().display());
}

fn find(directory: &Path, file: &str) -> Option<PathBuf> {
    for entry in std::fs::read_dir(directory).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            if let Some(path) = find(&path, file) {
                return Some(path);
            }
        } else if path.ends_with(file) {
            return Some(directory.into());
        }
    }
    None
}

fn run<F>(name: &str, mut configure: F) where F: FnMut(&mut Command) -> &mut Command {
    let mut command = Command::new(name);
    if !configure(&mut command).status().unwrap().success() {
        panic!("failed to execute {:?}", command);
    }
}
