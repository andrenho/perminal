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


// 
// Rust stuff
//
use std::env;

use plugin::Plugin;

pub struct PTY {
    fd: c_int,
}

impl PTY {
    pub fn new() -> PTY {
        let mut pty = PTY { fd: 0 };

        let shell = match env::var("SHELL") {
            Ok(v)  => v,
            Err(e) => "/bin/sh".to_string(),
        };

        let (pid, fd) = my_forkpty();
        match pid {
            -1 => panic!("Invalid return from forkpty!"),
            0  => {
                // child
                // TODO - set environment variables
                unsafe {
                    let mut argv = vec![CString::new("sh").unwrap().as_ptr()];
                    if execvp(CString::new(shell).unwrap().as_ptr(), argv.as_mut_ptr()) == -1 {
                        panic!("execlp");
                    }
                }
            },
            _  => {
                // self
                unsafe {
                    let flags = match fcntl(fd, F_GETFL, 0) {
                        -1  => 0,
                        v@_ => v
                    };
                    if fcntl(fd, F_SETFL, flags | O_NONBLOCK) == -1 {
                        panic!("fcntl");
                    }
                    pty.fd = fd;
                }
            }
        }

        pty
    }

}

impl Plugin for PTY {
    fn get(&self) -> Result<char, &'static str> {
        match unsafe {
            //let mut buf = CStr::new().unwrap().as_mut_ptr();
            let mut buf = ['\0' as c_char; 1];
            (read(self.fd, buf.as_mut_ptr() as *mut c_void, 1), buf[0])
        } { 
            c @ (1,_) => { panic!(); Ok(c.1 as u8 as char) },
            (_,_) => Ok('\0'),
        }
    }

    fn send(&self, c: char) -> Result<(), &'static str> {
        Ok(())
    }

    fn is_alive(&self) -> bool {
        true
    }
}

// vim: ts=4:sw=4:sts=4:expandtab
