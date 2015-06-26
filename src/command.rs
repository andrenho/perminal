pub enum Command {
    IncompleteCommand,

    //
    // CHARACTERS
    //
    PrintChar(char),
    
    //
    // SCREEN/CURSOR
    //
    CarriageReturn,
    CursorLeft,
    CursorDown,
    CursorRight,
    CursorUp,
    CursorHome,

    ClearScreen,
}

// vim: ts=4:sw=4:sts=4:expandtab
