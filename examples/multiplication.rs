extern crate libc;
extern crate tensorflux_sys as ffi;

use libc::{c_int, c_void, int64_t, size_t};
use std::ffi::{CStr, CString};
use std::path::Path;

macro_rules! nonnull(
    ($pointer:expr) => ({
        let pointer = $pointer;
        assert!(!pointer.is_null());
        pointer
    });
);

macro_rules! ok(
    ($status:expr) => ({
        if ffi::TF_GetCode($status) != ffi::TF_OK {
            panic!(CStr::from_ptr(ffi::TF_Message($status)).to_string_lossy().into_owned());
        }
    });
);

fn main() {
    use std::mem::size_of;
    use std::ptr::{null, null_mut};
    use std::slice::from_raw_parts;

    unsafe {
        let options = nonnull!(ffi::TF_NewSessionOptions());
        let status = nonnull!(ffi::TF_NewStatus());
        let session = nonnull!(ffi::TF_NewSession(options, status));

        let graph = read("examples/assets/multiplication.pb"); // c = a * b
        ffi::TF_ExtendGraph(session, graph.as_ptr() as *const _, graph.len() as size_t, status);
        ok!(status);

        let mut inputs = vec![];
        let mut input_values = vec![];

        let name = CString::new("a").unwrap();
        let mut data = vec![1f32, 2.0, 3.0];
        let dims = vec![data.len() as int64_t];
        let tensor = nonnull!(ffi::TF_NewTensor(ffi::TF_FLOAT, dims.as_ptr(), dims.len() as c_int,
                                                data.as_mut_ptr() as *mut _, data.len() as size_t,
                                                Some(noop), null_mut()));

        inputs.push(name.as_ptr());
        input_values.push(tensor);

        let name = CString::new("b").unwrap();
        let mut data = vec![4f32, 5.0, 6.0];
        let dims = vec![data.len() as int64_t];
        let tensor = nonnull!(ffi::TF_NewTensor(ffi::TF_FLOAT, dims.as_ptr(), dims.len() as c_int,
                                                data.as_mut_ptr() as *mut _, data.len() as size_t,
                                                Some(noop), null_mut()));

        inputs.push(name.as_ptr());
        input_values.push(tensor);

        let mut outputs = vec![];
        let mut output_values = vec![];

        let name = CString::new("c").unwrap();

        outputs.push(name.as_ptr());
        output_values.push(null_mut());

        let mut targets = vec![];

        ffi::TF_Run(session, null(), inputs.as_mut_ptr(), input_values.as_mut_ptr(),
                    inputs.len() as c_int, outputs.as_mut_ptr(), output_values.as_mut_ptr(),
                    outputs.len() as c_int, targets.as_mut_ptr(), targets.len() as c_int,
                    null_mut(), status);
        ok!(status);

        let tensor = nonnull!(output_values[0]);
        let data = nonnull!(ffi::TF_TensorData(tensor)) as *const f32;
        let data = from_raw_parts(data, ffi::TF_TensorByteSize(tensor) / size_of::<f32>());

        assert_eq!(data, &[1.0 * 4.0, 2.0 * 5.0, 3.0 * 6.0]);

        ffi::TF_CloseSession(session, status);

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
