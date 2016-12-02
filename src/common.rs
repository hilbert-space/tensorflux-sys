use libc::c_char;

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TF_DataType {
    TF_FLOAT = 1,
    TF_DOUBLE = 2,
    TF_INT32 = 3,
    TF_UINT8 = 4,
    TF_INT16 = 5,
    TF_INT8 = 6,
    TF_STRING = 7,
    TF_COMPLEX64 = 8,
    TF_INT64 = 9,
    TF_BOOL = 10,
    TF_QINT8 = 11,
    TF_QUINT8 = 12,
    TF_QINT32 = 13,
    TF_BFLOAT16 = 14,
    TF_QINT16 = 15,
    TF_QUINT16 = 16,
    TF_UINT16 = 17,
    TF_COMPLEX128 = 18,
    TF_HALF = 19,
    TF_RESOURCE = 20,
}
pub use TF_DataType::*;

extern {
    pub fn TF_Version() -> *const c_char;
}
