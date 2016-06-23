extern crate libc;
extern crate tensorflow_sys as ffi;

use libc::size_t;
use std::path::Path;

const GRAPH_PATH: &'static str = "tests/fixtures/graph.pb";

macro_rules! nonnull(
    ($pointer:expr) => ({
        let pointer = $pointer;
        assert!(!pointer.is_null());
        pointer
    });
);

macro_rules! success(
    ($status:expr) => (assert_eq!(ffi::TF_GetCode($status), ffi::TF_OK));
);

fn main() {
    unsafe {
        let options = nonnull!(ffi::TF_NewSessionOptions());
        let status = nonnull!(ffi::TF_NewStatus());
        let session = nonnull!(ffi::TF_NewSession(options, status));

        let graph = read(GRAPH_PATH);
        ffi::TF_ExtendGraph(session, graph.as_ptr() as *const _, graph.len() as size_t, status);
        success!(status);

        ffi::TF_DeleteSession(session, status);
        ffi::TF_DeleteStatus(status);
        ffi::TF_DeleteSessionOptions(options);
    }
}

fn read<T: AsRef<Path>>(path: T) -> Vec<u8> {
    use std::fs::File;
    use std::io::Read;

    let mut buffer = vec![];
    let mut file = File::open(path).unwrap();
    file.read_to_end(&mut buffer).unwrap();
    buffer
}
