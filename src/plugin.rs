use std::io::Error;

#[derive(Debug)]
pub enum TerminalError {
    NoData,
    EOF,
    Unexpected(Error)
}

pub trait Plugin {
    fn get(&self) -> Result<u8, TerminalError>;
    fn send(&self, c: u8) -> Result<(), TerminalError>;
    fn is_alive(&self) -> bool;
    fn term(&self) -> &str;
}

// vim: ts=4:sw=4:sts=4:expandtab
