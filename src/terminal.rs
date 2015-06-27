use std::cell::Cell;
use std::cell::RefCell;
use std::io::Error;

use config::Config;
use plugin::*;
use userevent::UserEvent;
use userevent::UserEvent::*;
use userevent::Key;
use matrix::Matrix;
use terminfo::Terminfo;
use terminfo_xterm256::TerminfoXterm256;

#[allow(dead_code)]  // TODO - remove this, is used for cfg
pub struct Terminal<'a, T: Plugin> {
    pub matrix: Matrix,
    cfg: &'a Config,
    plugin: T,
    terminfo: Box<Terminfo>,
    active: Cell<bool>,
    debug: RefCell<String>,
}

impl<'a, T:Plugin> Terminal<'a, T> {

    pub fn new(cfg: &'a Config, plugin: T) -> Terminal<T> { 
        let term = plugin.term();
        let t = Terminal { 
            matrix: Matrix::new(80, 25),  // TODO - size
            cfg: cfg,
            plugin: plugin, 
            active: Cell::new(true),
            terminfo: Terminal::<T>::terminfo(term),
            debug: RefCell::new(String::new()),
        };
        /*
        t.plugin.send('t' as u8);
        t.plugin.send('p' as u8);
        t.plugin.send('u' as u8);
        t.plugin.send('t' as u8);
        t.plugin.send(' ' as u8);
        t.plugin.send('c' as u8);
        t.plugin.send('u' as u8);
        t.plugin.send('d' as u8);
        t.plugin.send(' ' as u8);
        t.plugin.send('1' as u8);
        t.plugin.send('0' as u8);
        t.plugin.send(13);
        */
        t
    }


    pub fn is_alive(&self) -> bool { 
        self.active.get() && self.plugin.is_alive() 
    }


    pub fn user_input(&self, e: &UserEvent) -> Result<(), Error> {
        match e {
            &KeyPress { ref key, .. } => {
                match key {
                    /* Keys used to control the terminal, that
                       will not be sent to the plugin */
                    &Key::F12 => { self.active.set(false); Ok(()) }
                    /* The rest of the keys, that'll be sent
                       to the plugin */
                    key @ _ => {
                        if let &Key::Char(c) = key { 
                            self.debug('+', c as u8);
                        }
                        for k in self.terminfo.parse_input(key) {
                            match self.plugin.send(k) {
                                Err(e) => match e {
                                    TerminalError::Unexpected(ex) => return Err(ex),
                                    _ => unreachable!(),
                                },
                                _ => ()
                            }
                        }
                        Ok(())
                    },
                }
            },
            // Event::KeyRelease(_) => Ok(()),
        }
    }


    pub fn parse_plugin_output(&mut self) {
        loop {
            let commands = match self.plugin.get() {
                Ok(c)  => {
                    self.debug('-', c);
                    self.terminfo.parse_output(c)
                },
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


    pub fn print_debug_info(&self) {
        println!("{}", self.debug.borrow().to_string());
    }


    #[cfg(feature="debug_comm")]
    fn debug(&self, dir: char, c: u8) {
        let s = match c as u8 {
            0...31 => format!("{}[{}] ", dir, c as u8),
            _      => format!("{}[{} '{}'] ", dir, c as u8, c as char)
        };
        self.debug.borrow_mut().push_str(&s);
    }

    #[cfg(not(feature="debug_comm"))]
    #[allow(unused_variables)]
    fn debug(&self, dir: char, c: u8) {}

    fn terminfo(s: &str) -> Box<Terminfo> {
        match s {
            "xterm-256color" => Box::new(TerminfoXterm256::new()),
            s => panic!("Invalid terminal type '{}'", s)
        }
    }

}


// vim: ts=4:sw=4:sts=4:expandtab
