use libc::*;

pub type __time64_t = i64;
pub type _fsize_t = c_ulong;

// _finddata64i32_t
#[repr(C)]
pub struct _finddata_t {
    pub attrib: c_uint,
    pub time_create: __time64_t,
    pub time_access: __time64_t,
    pub time_write: __time64_t,
    pub size: _fsize_t,
    pub name: [c_char; 260],
}

extern {
    #[link_name="_findfirst64i32"]
    pub fn _findfirst(filespec: *const c_char, fileinfo: *mut _finddata_t) -> intptr_t;
    #[link_name="_findnext64i32"]
    pub fn _findnext(handle: intptr_t, fileinfo: *mut _finddata_t) -> c_int;
    #[link_name="_findclose"]
    pub fn _findclose(handle: intptr_t) -> c_int;
}
