use termcap::Termcap;
use command::Command;
use command::Command::*;

pub struct TermcapLinux;

impl TermcapLinux {

    pub fn new() -> TermcapLinux {
        TermcapLinux
    }

}

impl Termcap for TermcapLinux {
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
