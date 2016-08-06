use libc::{c_void, size_t};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TF_Buffer {
    pub data: *const c_void,
    pub length: size_t,
    pub data_deallocator: Option<unsafe extern fn(*mut c_void, size_t)>,
}

extern {
    pub fn TF_NewBuffer() -> *mut TF_Buffer;
    pub fn TF_NewBufferFromString(proto: *const c_void, proto_length: size_t) -> *mut TF_Buffer;
    pub fn TF_DeleteBuffer(buffer: *mut TF_Buffer);
    pub fn TF_GetBuffer(buffer: *mut TF_Buffer) -> TF_Buffer;
}
