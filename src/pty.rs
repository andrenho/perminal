//
// C stuff
//
extern crate libc;
use std::ptr;
use self::libc::*;
use std::ffi::CString;
use std::ffi::CStr;

#[link(name = "util")]
extern {
    fn forkpty(amaster: *mut c_int, name: *const c_int, x: *const c_int, y: *const c_int) -> pid_t;
}
fn my_forkpty() -> (pid_t, c_int) {
    unsafe { 
        let mut fd: [c_int; 1] = [0; 1];
        let pid = forkpty(fd.as_mut_ptr(), ptr::null(), ptr::null(), ptr::null());
        (pid, fd[0])
    }
}

fn main() {
}

// vim: ts=4:sw=4:sts=4:expandtab
