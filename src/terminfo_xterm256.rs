use terminfo::Terminfo;
use command::Command;
use command::Command::*;
use userevent::Key;
use userevent::Key::*;

pub struct TerminfoXterm256;

impl TerminfoXterm256 {

    pub fn new() -> TerminfoXterm256 {
        TerminfoXterm256
    }

}

impl Terminfo for TerminfoXterm256 {
    
    fn parse_output(&self, c: u8) -> Vec<Command> {
        match c {
            0 => vec![],
            10 => vec![LineFeed],
            13 => vec![CarriageReturn],
            c @ 1...255 => vec![PrintChar(c as char)],
            _ => panic!("Invalid value!"),
        }
    }

    fn parse_input(&self, key: &Key) -> Vec<u8>
    {
        match key {
            &Char(c) => vec![c as u8],
            _ => vec![],
        }
    }

}

// vim: ts=4:sw=4:sts=4:expandtab
