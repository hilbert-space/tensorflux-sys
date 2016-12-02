use libc::{c_char, size_t};

use TF_Status;

extern {
    pub fn TF_StringEncode(source: *const c_char, source_length: size_t, destination: *mut c_char,
                           destination_length: size_t, status: *mut TF_Status) -> size_t;
    pub fn TF_StringDecode(source: *const c_char, source_length: size_t,
                           destination: *mut *const c_char, destination_length: *mut size_t,
                           status: *mut TF_Status) -> size_t;
    pub fn TF_StringEncodedSize(length: size_t) -> size_t;
}
