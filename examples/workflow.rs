extern crate libc;
extern crate tensorflow_sys as ffi;

use libc::{c_int, c_longlong, c_void, size_t};
use std::ffi::{CStr, CString};
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
    ($status:expr) => ({
        if ffi::TF_GetCode($status) != ffi::TF_OK {
            panic!(CStr::from_ptr(ffi::TF_Message($status)).to_string_lossy().into_owned());
        }
    });
);

fn main() {
    unsafe {
        let options = nonnull!(ffi::TF_NewSessionOptions());
        let status = nonnull!(ffi::TF_NewStatus());
        let session = nonnull!(ffi::TF_NewSession(options, status));

        let graph = read(GRAPH_PATH);
        ffi::TF_ExtendGraph(session, graph.as_ptr() as *const _, graph.len() as size_t, status);
        success!(status);

        let mut input_names = vec![];
        let mut inputs = vec![];

        let name = CString::new("a:0").unwrap();
        let mut data = vec![1.0f32, 2.0f32];
        let mut dims = vec![data.len() as c_longlong];
        let tensor = nonnull!(ffi::TF_NewTensor(ffi::TF_FLOAT, dims.as_mut_ptr(),
                                                dims.len() as c_int, data.as_mut_ptr() as *mut _,
                                                data.len() as size_t, Some(noop), 0 as *mut _));

        input_names.push(name.as_ptr());
        inputs.push(tensor);

        let name = CString::new("b:0").unwrap();
        let mut data = vec![3.0f32, 4.0f32];
        let mut dims = vec![data.len() as c_longlong];
        let tensor = nonnull!(ffi::TF_NewTensor(ffi::TF_FLOAT, dims.as_mut_ptr(),
                                                dims.len() as c_int, data.as_mut_ptr() as *mut _,
                                                data.len() as size_t, Some(noop), 0 as *mut _));

        input_names.push(name.as_ptr());
        inputs.push(tensor);

        let mut output_names = vec![];
        let mut outputs = vec![];

        let name = CString::new("c:0").unwrap();

        output_names.push(name.as_ptr());
        outputs.push(0 as *mut _);

        let mut target_names = vec![];

        ffi::TF_Run(session, 0 as *const _, input_names.as_mut_ptr(), inputs.as_mut_ptr(),
                    input_names.len() as c_int, output_names.as_mut_ptr(), outputs.as_mut_ptr(),
                    output_names.len() as c_int, target_names.as_mut_ptr(),
                    target_names.len() as c_int, 0 as *mut _, status);
        success!(status);

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
