use libc::c_char;

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TF_Code {
    TF_OK = 0,
    TF_CANCELLED = 1,
    TF_UNKNOWN = 2,
    TF_INVALID_ARGUMENT = 3,
    TF_DEADLINE_EXCEEDED = 4,
    TF_NOT_FOUND = 5,
    TF_ALREADY_EXISTS = 6,
    TF_PERMISSION_DENIED = 7,
    TF_UNAUTHENTICATED = 16,
    TF_RESOURCE_EXHAUSTED = 8,
    TF_FAILED_PRECONDITION = 9,
    TF_ABORTED = 10,
    TF_OUT_OF_RANGE = 11,
    TF_UNIMPLEMENTED = 12,
    TF_INTERNAL = 13,
    TF_UNAVAILABLE = 14,
    TF_DATA_LOSS = 15,
}
pub use TF_Code::*;

#[derive(Clone, Copy, Debug)]
pub enum TF_Status {}

extern {
    pub fn TF_NewStatus() -> *mut TF_Status;
    pub fn TF_DeleteStatus(status: *mut TF_Status);
    pub fn TF_SetStatus(status: *mut TF_Status, code: TF_Code, message: *const c_char);
    pub fn TF_GetCode(status: *const TF_Status) -> TF_Code;
    pub fn TF_Message(status: *const TF_Status) -> *const c_char;
}
