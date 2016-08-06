extern crate libc;
extern crate tensorflux_sys as ffi;

fn main() {
    unsafe { compute() };
}

macro_rules! cstr(
    ($text:expr) => (CString::new($text).unwrap().as_ptr());
);

macro_rules! nonnull(
    ($pointer:expr) => ({
        let pointer = $pointer;
        assert!(!pointer.is_null());
        pointer
    });
);

unsafe fn compute() {
    use libc::{c_int, c_void, int64_t, size_t};
    use std::ffi::{CStr, CString};
    use std::mem::size_of;
    use std::ptr::{null, null_mut};
    use std::slice::from_raw_parts;

    let status = nonnull!(ffi::TF_NewStatus());

    macro_rules! ok(
        ($result:expr) => ({
            let result = $result;
            if ffi::TF_GetCode(status) != ffi::TF_OK {
                panic!(CStr::from_ptr(ffi::TF_Message(status)).to_string_lossy().into_owned());
            }
            result
        });
    );

    let graph = nonnull!(ffi::TF_NewGraph());

    let description = nonnull!(ffi::TF_NewNode(graph, cstr!("Placeholder"), cstr!("a")));
    ffi::TF_SetAttrType(description, cstr!("dtype"), ffi::TF_FLOAT);
    let a = ok!(ffi::TF_FinishNode(description, status));
    let a = ffi::TF_Port { node: a, index: 0 };

    let description = nonnull!(ffi::TF_NewNode(graph, cstr!("Placeholder"), cstr!("b")));
    ffi::TF_SetAttrType(description, cstr!("dtype"), ffi::TF_FLOAT);
    let b = ok!(ffi::TF_FinishNode(description, status));
    let b = ffi::TF_Port { node: b, index: 0 };

    let description = nonnull!(ffi::TF_NewNode(graph, cstr!("Mul"), cstr!("c")));
    ffi::TF_AddInput(description, a);
    ffi::TF_AddInput(description, b);
    let c = ok!(ffi::TF_FinishNode(description, status));
    let c = ffi::TF_Port { node: c, index: 0 };

    let mut inputs = vec![];
    let mut input_values = vec![];

    let mut data = vec![1f32, 2.0, 3.0];
    let dims = vec![data.len() as int64_t];
    let tensor = nonnull!(ffi::TF_NewTensor(ffi::TF_FLOAT, dims.as_ptr(), dims.len() as c_int,
                                            data.as_mut_ptr() as *mut _, data.len() as size_t,
                                            Some(noop), null_mut()));

    inputs.push(a);
    input_values.push(tensor);

    let mut data = vec![4f32, 5.0, 6.0];
    let dims = vec![data.len() as int64_t];
    let tensor = nonnull!(ffi::TF_NewTensor(ffi::TF_FLOAT, dims.as_ptr(), dims.len() as c_int,
                                            data.as_mut_ptr() as *mut _, data.len() as size_t,
                                            Some(noop), null_mut()));

    inputs.push(b);
    input_values.push(tensor);

    let mut outputs = vec![];
    let mut output_values = vec![];

    outputs.push(c);
    output_values.push(null_mut());

    let targets = vec![];

    let options = nonnull!(ffi::TF_NewSessionOptions());
    let session = ok!(ffi::TF_NewSessionWithGraph(graph, options, status));

    ok!(ffi::TF_SessionRun(session, null(), inputs.as_ptr(), input_values.as_ptr(),
                           inputs.len() as c_int, outputs.as_ptr(), output_values.as_mut_ptr(),
                           outputs.len() as c_int, targets.as_ptr(), targets.len() as c_int,
                           null_mut(), status));

    let tensor = nonnull!(output_values[0]);
    let data = nonnull!(ffi::TF_TensorData(tensor)) as *const f32;
    let data = from_raw_parts(data, ffi::TF_TensorByteSize(tensor) / size_of::<f32>());

    assert_eq!(data, &[1.0 * 4.0, 2.0 * 5.0, 3.0 * 6.0]);

    ok!(ffi::TF_CloseSessionWithGraph(session, status));
    ok!(ffi::TF_DeleteSessionWithGraph(session, status));
    ffi::TF_DeleteTensor(tensor);
    ffi::TF_DeleteStatus(status);
    ffi::TF_DeleteSessionOptions(options);

    unsafe extern "C" fn noop(_: *mut c_void, _: size_t, _: *mut c_void) {}
}
