use libc::{c_int, c_void, int64_t, size_t};

use TF_DataType;

#[derive(Clone, Copy, Debug)]
pub enum TF_Tensor {}

extern {
    pub fn TF_NewTensor(data_type: TF_DataType,
                        dims: *const int64_t,
                        num_dims: c_int,
                        data: *mut c_void,
                        length: size_t,
                        deallocator: Option<unsafe extern fn(*mut c_void, size_t, *mut c_void)>,
                        deallocator_argument: *mut c_void)
                        -> *mut TF_Tensor;
    pub fn TF_AllocateTensor(data_type: TF_DataType,
                             dims: *const int64_t,
                             num_dims: c_int,
                             length: size_t)
                             -> *mut TF_Tensor;
    pub fn TF_DeleteTensor(tensor: *mut TF_Tensor);
    pub fn TF_TensorType(tensor: *const TF_Tensor) -> TF_DataType;
    pub fn TF_NumDims(tensor: *const TF_Tensor) -> c_int;
    pub fn TF_Dim(tensor: *const TF_Tensor, index: c_int) -> int64_t;
    pub fn TF_TensorByteSize(tensor: *const TF_Tensor) -> size_t;
    pub fn TF_TensorData(tensor: *const TF_Tensor) -> *mut c_void;
}
