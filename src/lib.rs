//! Binding to [TensorFlow][1].
//!
//! [1]: https://www.tensorflow.org

#![allow(non_camel_case_types)]

extern crate libc;

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

mod buffer;
mod graph;
mod library;
mod node;
mod session;
mod status;
mod tensor;

pub use buffer::*;
pub use graph::*;
pub use library::*;
pub use node::*;
pub use session::*;
pub use status::*;
pub use tensor::*;
