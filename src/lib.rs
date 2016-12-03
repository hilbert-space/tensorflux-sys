//! Binding to [TensorFlow][1].
//!
//! [1]: https://www.tensorflow.org

#![allow(non_camel_case_types)]

extern crate libc;

mod buffer;
mod graph;
mod library;
mod session;
mod status;
mod string;
mod tensor;

pub use buffer::*;
pub use graph::*;
pub use library::*;
pub use session::*;
pub use status::*;
pub use string::*;
pub use tensor::*;

use libc::c_char;

extern {
    pub fn TF_Version() -> *const c_char;
}
