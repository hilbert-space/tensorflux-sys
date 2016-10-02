use libc::c_char;

use {TF_Buffer, TF_Status};

#[derive(Clone, Copy, Debug)]
pub enum TF_Library {}

extern {
    pub fn TF_LoadLibrary(name: *const c_char, status: *mut TF_Status) -> *mut TF_Library;
    pub fn TF_DeleteLibraryHandle(library: *mut TF_Library);
    pub fn TF_GetOpList(library: *mut TF_Library) -> TF_Buffer;
    pub fn TF_GetAllOpList() -> *mut TF_Buffer;
}
