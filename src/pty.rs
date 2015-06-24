//
// C stuff
//
extern crate libc;
use std::ptr;
use self::libc::*;
use std::ffi::CString;

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
use std::io::Error;
use std::cell::Cell;

use plugin::*;

#[allow(dead_code)]
pub struct PTY {
    pid: pid_t,
    fd: c_int,
    alive: Cell<bool>,
}

impl PTY {
    pub fn new() -> PTY {
        let shell = match env::var("SHELL") {
            Ok(v)  => v,
            Err(_) => "/bin/sh".to_string(),
        };

        let (pid, fd) = my_forkpty();
        match pid {
            -1 => panic!("Invalid return from forkpty!"),
            0  => {
                // child
                // TODO - set environment variables
                unsafe {
                    let shell = CString::new(shell).unwrap();
                    let mut argv = vec![CString::new("sh").unwrap().as_ptr()];
                    if execvp(shell.as_ptr(), argv.as_mut_ptr()) == -1 {
                        panic!("execvp");
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
                }
            }
        }

        PTY { fd: fd, pid: pid, alive: Cell::new(true), }
    }
}


impl Plugin for PTY {

    fn get(&self) -> Result<u8, TerminalError> {
        match unsafe {
            let mut buf = ['\0' as c_char; 1];
            (read(self.fd, buf.as_mut_ptr() as *mut c_void, 1), buf[0])
        } { 
            c @ (1,_) => Ok(c.1 as u8),
            (0,_)     => { self.alive.set(false); Err(TerminalError::EOF) },
            (-1,_)    => { 
                let err = Error::last_os_error();
                match err.raw_os_error() {
                    Some(EAGAIN) => Err(TerminalError::NoData),
                    Some(EIO)    => { self.alive.set(false); Err(TerminalError::EOF) },
                    Some(_)      => Err(TerminalError::Unexpected(err)),
                    _            => unreachable!(),
                }
            },
            (_,_)     => unreachable!()
        }
    }


    fn send(&self, c: u8) -> Result<(), TerminalError> {
        let buf = [c as c_char; 1];
        match unsafe { write(self.fd, buf.as_ptr() as *const c_void, 1) } {
            -1...0 => Err(TerminalError::Unexpected(Error::last_os_error())),
            1 => Ok(()),
            _ => unreachable!(),
        }
    }


    fn is_alive(&self) -> bool {
        self.alive.get()
    }
    
}


impl Drop for PTY {
    fn drop(&mut self) {
        unsafe { close(self.fd); }
    }
}

// vim: ts=4:sw=4:sts=4:expandtab
