use command::Command;

pub trait Termcap {
    fn parse(&self, c: u8) -> Vec<Command>;
}

// vim: ts=4:sw=4:sts=4:expandtab
