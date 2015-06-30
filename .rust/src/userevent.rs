pub enum Key {
    F12,
    Char(char),
}

pub enum UserEvent {
    KeyPress { key: Key, control: bool, shift: bool, alt: bool },
//    KeyRelease(Key),
}


// vim: ts=4:sw=4:sts=4:expandtab
