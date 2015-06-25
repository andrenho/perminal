use termcap::Termcap;
use command::Command;
use command::Command::*;

pub struct TermcapXterm256;

impl TermcapXterm256 {

    pub fn new() -> TermcapXterm256 {
        TermcapXterm256
    }

}

impl Termcap for TermcapXterm256 {
    fn parse(&self, c: u8) -> Vec<Command> {
        match c {
            0 => vec![],
            10 => vec![LineFeed],
            13 => vec![CarriageReturn],
            c @ 1...255 => vec![PrintChar(c as char)],
            _ => panic!("Invalid value!"),
        }
    }
}

// vim: ts=4:sw=4:sts=4:expandtab
