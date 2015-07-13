pub enum SpecialKey {
    F12,
}

pub enum UserEvent {
    KeyPress(char),
    SpecialKeyPress(SpecialKey),
}

// vim: ts=4:sw=4:sts=4:expandtab
