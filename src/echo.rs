use std::cell::RefCell;
use plugin::Plugin;

pub struct Echo { 
    last: RefCell<Vec<char>>
}

impl Echo {
    pub fn new() -> Echo {
        Echo { 
            last: RefCell::new(Vec::new()),
        }
    }
}

impl Plugin for Echo {
    fn get(&self) -> Result<char, &'static str> { 
        Ok(match self.last.borrow_mut().pop() {
            None => '\0',
            Some(c) => c,
        })
    }

    fn send(&self, c: char) -> Result<(), &'static str> { 
        self.last.borrow_mut().insert(0, c);
        Ok(())
    }

    fn is_alive(&self) -> bool { 
        true 
    }
}

// vim: ts=4:sw=4:sts=4:expandtab
