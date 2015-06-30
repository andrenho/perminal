use command::Command;
use userevent::Key;

pub trait Terminfo {
    fn parse_output(&mut self, c: u8) -> Vec<Command>;
    fn parse_input(&self, key: &Key) -> Vec<u8>;
}

// vim: ts=4:sw=4:sts=4:expandtab
