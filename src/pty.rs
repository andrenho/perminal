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

#[allow(dead_code)]
pub struct PTY {
    pid: pid_t,
    fd: c_int,
}

impl PTY {

    pub fn new(shell: Option<&'static str>, login: bool) -> PTY {
        let shell = match shell {
            Some(s) => s.to_string(),
            None => match env::var("SHELL") {
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
                unsafe {
                    let sh = CString::new(shell.clone()).unwrap();
                    let mut argv = vec![CString::new(if login { "" } else { "sh" }).unwrap().as_ptr()];
                    if execv(sh.as_ptr(), argv.as_mut_ptr()) == -1 {
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

        PTY { fd: fd, pid: pid, }
    }

    
    fn read(&self, data: &mut Vec<u8>) -> Option<usize> {
        const sz: usize = 1024 * 32;
        let mut buf = ['\0' as c_char; sz];  // TODO - don't allocate each time
        match unsafe { read(self.fd, buf.as_mut_ptr() as *mut c_void, sz as u64) } {
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
                for i in 0..(n as usize) {
                    data.push(buf[i] as u8);  // TODO - this is slow!
                }
                Some(n as usize)
            }
        }
    }


    fn write(&self, data: &Vec<u8>) {
        if !unsafe { write(self.fd, data.as_ptr() as *const c_void, data.len() as u64) } == -1 {
            panic!("There was an error writing to the PTY");
        }
    }

}


/*
        let buf = [c as c_char; 1];
        match unsafe { write(self.fd, buf.as_ptr() as *const c_void, 1) } {
            -1...0 => Err(TerminalError::Unexpected(Error::last_os_error())),
            1 => Ok(()),
            _ => unreachable!(),
        }
*/
/*
    if(write(fd, data, n) == -1) {
        perror("write");
        throw PluginException("There was an error writing to the PTY");
    }
*/

/*
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
    
    fn term(&self) -> &'static str { "xterm-256color" }
}
*/


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
        let p = PTY::new(Some("/bin/sh"), false);

        // read
        let mut n;
        loop {
            n = p.read(&mut data).unwrap();
            if n > 0 { break; }
        }
        unsafe { println!("{:?}", String::from_utf8_unchecked(data.clone())); }
        assert_eq!(data, vec![97u8, 98u8, 99u8, 100u8, 101u8]);

        // write
        data.clear();
        p.write(&vec!['x' as u8, 'y' as u8]);
        loop {
            n = p.read(&mut data).unwrap();
            if n > 0 { break; }
        }
        assert_eq!(data, vec!['x' as u8, 'y' as u8]);
    }

}

// vim: ts=4:sw=4:sts=4:expandtab
