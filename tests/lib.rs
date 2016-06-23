extern crate tensorflow_sys as raw;

#[test]
fn link() {
    unsafe {
        let buffer = raw::TF_NewBuffer();
        assert!(!buffer.is_null());
        raw::TF_DeleteBuffer(buffer);
    }
}
