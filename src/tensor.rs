use libc::{c_int, c_void, int64_t, size_t};

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
}
pub use TF_DataType::*;

#[derive(Clone, Copy, Debug)]
pub enum TF_Tensor {}

extern {
    pub fn TF_NewTensor(data_type: TF_DataType, dims: *const int64_t, ndims: c_int,
                        data: *mut c_void, length: size_t,
                        deallocator: Option<unsafe extern fn(*mut c_void, size_t, *mut c_void)>,
                        deallocator_argument: *mut c_void) -> *mut TF_Tensor;
    pub fn TF_AllocateTensor(data_type: TF_DataType, dims: *const int64_t, ndims: c_int,
                             length: size_t) -> *mut TF_Tensor;
    pub fn TF_DeleteTensor(tensor: *mut TF_Tensor);
    pub fn TF_TensorType(tensor: *const TF_Tensor) -> TF_DataType;
    pub fn TF_NumDims(tensor: *const TF_Tensor) -> c_int;
    pub fn TF_Dim(tensor: *const TF_Tensor, index: c_int) -> int64_t;
    pub fn TF_TensorByteSize(tensor: *const TF_Tensor) -> size_t;
    pub fn TF_TensorData(tensor: *const TF_Tensor) -> *mut c_void;
}
