extern crate pkg_config;

use std::path::{Path, PathBuf};
use std::process::Command;

const NAME: &'static str = "tensorflow";
const SOURCE_URL: &'static str = "https://github.com/tensorflow/tensorflow.git";

macro_rules! get(($name:expr) => (::std::env::var($name).unwrap()));

fn main() {
    if pkg_config::find_library(NAME).is_ok() {
        return;
    }

    let source = PathBuf::from(&get!("CARGO_MANIFEST_DIR")).join("source");
    if !Path::new(&source.join(".git")).exists() {
        Command::new("git").args(&["clone", "--recurse-submodules", SOURCE_URL])
                           .arg(source).status().unwrap();
    }
}
