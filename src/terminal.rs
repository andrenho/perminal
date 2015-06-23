use std::cell::Cell;

use plugin::Plugin;
use userevent::UserEvent;
use userevent::UserEvent::*;
use userevent::Key;
use command::Command::*;
use matrix::Matrix;

pub struct Terminal<'a> {
    pub matrix: Matrix,
    plugin: &'a (Plugin + 'a),
    active: Cell<bool>,
    // current: Cell<Vec<char>>,
}

impl<'a> Terminal<'a> {
    pub fn new(plugin: &'a Plugin) -> Terminal<'a> { 
        Terminal { 
            plugin: plugin, 
            active: Cell::new(true),
            matrix: Matrix::new(80, 25),  // TODO - size
        }
    }

    pub fn is_alive(&self) -> bool { 
        self.active.get() && self.plugin.is_alive() 
    }

    pub fn send(&self, e: &UserEvent) -> Result<(), &'static str> {
        match e {
            &KeyPress { ref key, .. } => {
                match key {
                    &Key::F12 => { self.active.set(false); Ok(()) }
                    &Key::Char(k) => self.plugin.send(k),
                }
            },
            // Event::KeyRelease(_) => Ok(()),
        }
    }

    pub fn parse_plugin_output(&mut self) {
        loop {
            match self.plugin.get() {
                Ok(c) => {
                    match c as u8 {
                        0 => break,
                        c @ 1...255 => self.matrix.execute(PrintChar(c as char)),
                        _ => panic!("Invalid value!"),
                    }
                },
                Err(s) => panic!(s),
            }
        }
        self.matrix.update_cursor();
    }
}

// vim: ts=4:sw=4:sts=4:expandtab
