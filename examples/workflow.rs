extern crate libc;
extern crate tensorflow_sys as ffi;

use libc::{c_int, c_longlong, c_void, size_t};
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

        let mut data = vec![69.0, 42.0];
        let mut dims = vec![2 as c_longlong];
        let tensor = nonnull!(ffi::TF_NewTensor(ffi::TF_DOUBLE, dims.as_mut_ptr(),
                                                dims.len() as c_int, data.as_mut_ptr() as *mut _,
                                                data.len() as size_t, Some(noop), 0 as *mut _));

        ffi::TF_DeleteTensor(tensor);
        ffi::TF_DeleteSession(session, status);
        ffi::TF_DeleteStatus(status);
        ffi::TF_DeleteSessionOptions(options);
    }

    unsafe extern "C" fn noop(_: *mut c_void, _: size_t, _: *mut c_void) {}
}

fn read<T: AsRef<Path>>(path: T) -> Vec<u8> {
    use std::fs::File;
    use std::io::Read;

    let mut buffer = vec![];
    let mut file = File::open(path).unwrap();
    file.read_to_end(&mut buffer).unwrap();
    buffer
}
