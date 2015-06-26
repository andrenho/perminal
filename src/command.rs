pub enum Command {
    IncompleteCommand,

    //
    // CHARACTERS
    //
    PrintChar(char),
    
    //
    // SCREEN/CURSOR
    //

    // local cursor movement
    CarriageReturn,
    CursorLeft,
    CursorDown,
    CursorRight,
    CursorUp,
    CursorHome,
    
    // parameter cursor movement
    CursorPDown(u16),

    ClearScreen,
}

// vim: ts=4:sw=4:sts=4:expandtab
