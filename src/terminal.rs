use std::cell::Cell;
use std::io::Error;

use plugin::*;
use userevent::UserEvent;
use userevent::UserEvent::*;
use userevent::Key;
use matrix::Matrix;
use termcap::Termcap;
use termcap_linux::TermcapLinux;

pub struct Terminal<'a> {
    pub matrix: Matrix,
    plugin: &'a (Plugin + 'a),
    active: Cell<bool>,
    termcap: Box<Termcap>,
}

impl<'a> Terminal<'a> {

    pub fn new(plugin: &'a Plugin, term_variable: &'a str) -> Terminal<'a> { 
        Terminal { 
            matrix: Matrix::new(80, 25),  // TODO - size
            plugin: plugin, 
            active: Cell::new(true),
            termcap: Terminal::termcap(term_variable),
        }
    }


    pub fn is_alive(&self) -> bool { 
        self.active.get() && self.plugin.is_alive() 
    }


    pub fn user_input(&self, e: &UserEvent) -> Result<(), Error> {
        match e {
            &KeyPress { ref key, .. } => {
                match key {
                    &Key::F12 => { self.active.set(false); Ok(()) }
                    &Key::Char(k) => { //{ self.plugin.send(k as u8); Ok(()) }
                        match self.plugin.send(k as u8) {
                            Ok(_) => Ok(()),
                            Err(e) => match e {
                                TerminalError::Unexpected(ex) => Err(ex),
                                _ => unreachable!(),
                            }
                        }
                    }
                }
            },
            // Event::KeyRelease(_) => Ok(()),
        }
    }


    pub fn parse_plugin_output(&mut self) {
        loop {
            let commands = match self.plugin.get() {
                Ok(c) => self.termcap.parse(c),
                /*{
                    match c as u8 {
                        0 => vec!,
                        10 => self.matrix.execute(LineFeed),
                        13 => self.matrix.execute(CarriageReturn),
                        c @ 1...255 => self.matrix.execute(PrintChar(c as char)),
                        _ => panic!("Invalid value!"),
                    }
                },*/
                Err(e) => match e {
                    TerminalError::Unexpected(e) => panic!(e),
                    _ => break,
                }
            };
            for cmd in commands.iter() {
                self.matrix.execute(cmd);
            }
        }
        self.matrix.update_cursor();
    }


    fn termcap(s: &str) -> Box<Termcap> {
        match s {
            "linux" => Box::new(TermcapLinux::new()),
            s => panic!("Invalid terminal type '{}'", s)
        }
    }


}


// vim: ts=4:sw=4:sts=4:expandtab
