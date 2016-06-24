extern crate pkg_config;

use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

const FILENAME: &'static str = "libtensorflow.so";
const LIBRARY: &'static str = "tensorflow";
const REPOSITORY: &'static str = "https://github.com/tensorflow/tensorflow.git";
const VERSION: &'static str = "0.9.0rc0";

macro_rules! get(($name:expr) => (env::var($name).unwrap()));

fn main() {
    if pkg_config::find_library(LIBRARY).is_ok() {
        return;
    }

    let output = PathBuf::from(&get!("OUT_DIR"));
    if !output.join(FILENAME).exists() {
        let source = PathBuf::from(&get!("CARGO_MANIFEST_DIR")).join("source");
        if !Path::new(&source.join(".git")).exists() {
            run("git", |command| command.arg("clone")
                                        .arg(format!("--branch=v{}", VERSION))
                                        .arg("--recursive")
                                        .arg(REPOSITORY)
                                        .arg(&source));
        }

        run("./configure", |command| command.current_dir(&source));
        run("bazel", |command| command.current_dir(&source)
                                      .arg("build")
                                      .arg(format!("--jobs={}", get!("NUM_JOBS")))
                                      .arg("--compilation_mode=opt")
                                      .arg(format!("{}:{}", LIBRARY, FILENAME)));

        fs::copy(find(&source, FILENAME).unwrap(), output.join(FILENAME)).unwrap();
    }

    println!("cargo:rustc-link-lib=dylib={}", LIBRARY);
    println!("cargo:rustc-link-search={}", output.display());
}

fn find(directory: &Path, file: &str) -> Option<PathBuf> {
    for entry in fs::read_dir(directory).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            if let Some(path) = find(&path, file) {
                return Some(path);
            }
        } else {
            if path.is_file() && path.ends_with(file) {
                return Some(path);
            }
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
