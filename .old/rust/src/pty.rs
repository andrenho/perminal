//
// C stuff
//
extern crate libc;
use self::libc::*;
use self::libc::funcs::bsd44::ioctl;

use std::ptr;
use std::ffi::CString;

#[link(name = "util")]
extern {
    fn forkpty(amaster: *mut c_int, name: *const c_char, x: *const c_void, y: *const c_void) -> pid_t;
    fn execlp(file: *const c_char, args: *const c_char, ...) -> c_int;
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

#[allow(dead_code)]
pub struct PTY {
    pid: pid_t,
    fd: c_int,
}

impl PTY {

    pub fn new(testing: bool) -> PTY {
        let shell = if testing {
            "/bin/sh".to_string()
        } else {
            match env::var("SHELL") {
                Ok(v)  => v,
                Err(_) => "/bin/sh".to_string(),
            }
        };
        let (pid, fd) = my_forkpty();
        match pid {
            -1 => panic!("Invalid return from forkpty!"),
            0  => {
                // child
                env::set_var("TERM", "perminal");
                if !testing { 
                    PTY::print_motd(); 
                }
                unsafe {
                    let sh = CString::new(shell).unwrap();
                    let arg1 = CString::new(if testing { "sh" } else { "" }).unwrap();
                    if execlp(sh.as_ptr(), arg1.as_ptr(), ptr::null::<*const c_char>()) == -1 {
                        panic!("execv");
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

        PTY { fd: fd, pid: pid, }
    }

    
    pub fn read(&self, data: &mut Vec<u8>) -> Option<usize> {
        const SZ: usize = 1024 * 32;
        let mut buf = ['\0' as c_char; SZ];  // TODO - don't allocate each time
        match unsafe { read(self.fd, buf.as_mut_ptr() as *mut c_void, SZ as c_ulong) } {
            -1  => { 
                let err = Error::last_os_error();
                match err.raw_os_error() {
                    Some(EAGAIN) => Some(0),  // no data from socket (socket is O_NONBLOCK)
                    Some(EIO) => None,        // the connection was cut
                    _ => panic!("There was an error reading from the PTY."),
                }
            },
            0   => None,  // the connection ended
            n@_ => {
                let slice = &buf[..(n as usize)];
                data.extend(slice.iter().map(|&d| d as u8));
                Some(n as usize)
            }
        }
    }


    pub fn write(&self, data: &Vec<u8>) {
        if !unsafe { write(self.fd, data.as_ptr() as *const c_void, data.len() as c_ulong) } == -1 {
            panic!("There was an error writing to the PTY");
        }
    }


    pub fn resize(&self, w: u16, h: u16) {
        // ideas from <http://hermanradtke.com/2015/01/12/terminal-window-size-with-rust-ffi.html>
        const TIOCSWINSZ : c_ulong = 0x5414;
        #[repr(C)]  // TODO - packed?
        struct winsize {
            ws_row: c_ushort,
            ws_col: c_ushort,
            ws_xpixel: c_ushort,
            ws_ypixel: c_ushort,
        }
        let ws = winsize { 
            ws_row: h as c_ushort, 
            ws_col: w as c_ushort, 
            ws_xpixel: 0, 
            ws_ypixel: 0 
        };
        if unsafe { ioctl(self.fd, TIOCSWINSZ, &ws) } < 0 {
            panic!("Couldn't set window size!");
        }
    }


    fn print_motd() {
        println!("`perminal` aims to be a very fast, small, highly compliant and highly");
        println!("configurable terminal emulator for various operating systems, with multiple");
        println!("front-ends and multiple-backends.");
        println!("");
        println!("Right now, it is niether of these things. This is a very alpha release, so");
        println!("DO NOT USE IN PRODUCTION ENVIRONMENTS.");
        println!("");
    }

}


impl Drop for PTY {
    fn drop(&mut self) {
        unsafe { close(self.fd); }
    }
}



//
// TESTS
//
#[cfg(test)]
mod tests {

    use super::PTY;
    use std::env;

    #[test]
    fn test_pty() {
        // open conection
        let mut data: Vec<u8> = Vec::new();
        env::set_var("PS1", "abcde");
        let p = PTY::new(true);

        // read
        let mut n;
        loop {
            n = p.read(&mut data).unwrap();
            if n > 0 { break; }
        }
        unsafe { println!("return from `read`: {:?}", String::from_utf8_unchecked(data.clone())); }
        assert_eq!(data, vec![97u8, 98u8, 99u8, 100u8, 101u8]);

        // write
        data.clear();
        p.write(&"a".bytes().collect());
        loop {
            n = p.read(&mut data).unwrap();
            if n > 0 { break; }
        }
        unsafe { println!("return from `read`: {:?}", String::from_utf8_unchecked(data.clone())); }
        assert_eq!(data, "a".bytes().collect::<Vec<u8>>());

        // resize
        data.clear();
        p.resize(10, 10);
    }

}

// vim: ts=4:sw=4:sts=4:expandtab
