pub trait Plugin {
    fn get(&self) -> Result<char, &'static str>;
    fn send(&self, c: char) -> Result<(), &'static str>;
    fn is_alive(&self) -> bool;
}

// vim: ts=4:sw=4:sts=4:expandtab
