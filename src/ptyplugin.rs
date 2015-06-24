use plugin::Plugin;
use pty::PTY;

pub struct PTYPlugin {
    alive: bool,
    pty: PTY,
}

impl PTYPlugin {
    pub fn new() -> PTYPlugin {
        PTYPlugin { 
            alive: true,
            pty: PTY::new(),
        }
    }
}

impl Plugin for PTYPlugin {
    fn get(&self) -> Result<char, &'static str> {
        match self.pty.getc() {
            Ok(c) => Ok(c as char),
            Err(_) => Ok('\0'),
        }
    }

    fn send(&self, c: char) -> Result<(), &'static str> {
        match self.pty.send(c as u8) {
            Ok(_) => Ok(()),
            _ => Ok(())
        }
    }

    fn is_alive(&self) -> bool {
        self.alive
    }
}

// vim: ts=4:sw=4:sts=4:expandtab
