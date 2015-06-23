use std::cell::Cell;
use plugin::Plugin;

pub struct Echo { 
    last: Cell<char> 
}

impl Echo {
    pub fn new() -> Echo {
        Echo { last: Cell::new('\0') }
    }
}

impl Plugin for Echo {
    fn get(&self) -> Result<char, &'static str> { 
        let c = self.last.get();
        self.last.set('\0');
        Ok(c) 
    }

    fn send(&self, c: char) -> Result<(), &'static str> { 
        self.last.set(c);
        Ok(())
    }

    fn is_alive(&self) -> bool { true }
}

// vim: ts=4:sw=4:sts=4:expandtab
