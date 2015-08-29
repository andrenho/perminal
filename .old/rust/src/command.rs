#[derive(PartialEq, Eq, Debug)]
pub enum Command {
    //
    // CHARACTERS
    //
    PrintChar(Vec<u8>),
    InvalidUtf8,
    
    //
    // SCREEN/CURSOR
    //

    // local cursor movement
//  CarriageReturn,
//  CursorLeft,
    CursorRight,
//  CursorDown,
//  CursorUp,
//  CursorHome,
    
    // parameter cursor movement
//  CursorPDown(u16),
//  CursorPUp(u16),
//  CursorPLeft(u16),
//  CursorPRight(u16),

    // absolute cursor movement
//  SaveCursorPosition,
//  RestoreCursorPosition,
//  MoveCursor(u16,u16),
//  MoveCursorColumn(u16),
//  MoveCursorRow(u16),

    // scrolling
    ChangeScrollRegion(u16,u16),
//  ScrollForward(u16),
//  ScrollReverse(u16),

    // add to screen
//  InsertLine,
//  InsertLines(u16),

    // delete from screen
//  ClearScreen,
//  DeleteChar,
//  DeleteChars(u16),
//  DeleteLine,
//  DeleteLines(u16),
//  EraseChars(u16),
//  ClearEOS,
//  ClearEOL,
//  ClearBOL,

    // insert mode
//  SetInsertMode(bool),
//  InsertChars(u16),

    // attributes
//  SetStandoutMode(bool),
//  SetUnderlineMode(bool),
//  SetBlinkMode,
//  SetBoldMode,
//  SetInvisibleMode,
//  #[allow(dead_code)] SetReverseMode,
//  ExitAttributeMode,
//  SetCharsetMode(bool),

    // bell
//  Bell,
//  ReverseScreen(bool),

    // cursor intensity
//  CursorVisibility(u16),

    // meta mode
//  SetMetaMode(bool),

    // program initialization
//  SaveScreen,
//  RestoreScreen,

    // keypad activation
//  SetKeypadMode(bool),
}

// vim: ts=4:sw=4:sts=4:expandtab
