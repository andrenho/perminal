use std::cell::Cell;
use std::io::Error;

use config::Config;
use plugin::*;
use userevent::UserEvent;
use userevent::UserEvent::*;
use userevent::Key;
use matrix::Matrix;
use terminfo::Terminfo;
use terminfo_xterm256::TerminfoXterm256;

pub struct Terminal<'a> {
    pub matrix: Matrix,
    cfg: &'a Config,
    plugin: &'a (Plugin + 'a),
    active: Cell<bool>,
    terminfo: Box<Terminfo>,
}

impl<'a> Terminal<'a> {

    pub fn new(cfg: &'a Config, plugin: &'a Plugin) -> Terminal<'a> { 
        Terminal { 
            matrix: Matrix::new(80, 25),  // TODO - size
            cfg: cfg,
            plugin: plugin, 
            active: Cell::new(true),
            terminfo: Terminal::terminfo(plugin.term()),
        }
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
                Ok(c)  => self.terminfo.parse_output(c),
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


    fn terminfo(s: &str) -> Box<Terminfo> {
        match s {
            "xterm-256color" => Box::new(TerminfoXterm256::new()),
            s => panic!("Invalid terminal type '{}'", s)
        }
    }


}


// vim: ts=4:sw=4:sts=4:expandtab
