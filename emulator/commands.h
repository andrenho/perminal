#ifndef COMMANDS_H
#define COMMANDS_H

enum Command {
    NONE,
    IGNORE,
    REGULAR_INPUT,
    UNWIND,  // called when it was not a command after all

    TAB,  // TODO - ?

    // local cursor movement
    CARRIAGE_RETURN,
    CURSOR_LEFT,
    CURSOR_DOWN,
    CURSOR_RIGHT,
    CURSOR_UP,
    CURSOR_HOME,
    CURSOR_LL,
    
    // parameter cursor movement
    CURSORP_DOWN,
    CURSORP_UP,
    CURSORP_LEFT,
    CURSORP_RIGHT,

    // absolute cursor movement
    SAVE_CURSOR_POSITION,
    RESTORE_CURSOR_POSITION,
    MOVE_CURSOR,
    MOVE_CURSOR_COLUMN,
    MOVE_CURSOR_ROW,

    // scrolling
    CHANGE_SCROLL_REGION,
    SCROLL_FORWARD,
    SCROLL_REVERSE,

    // add to screen
    INSERT_LINE,
    INSERT_LINES,

    // delete from screen
    CLEAR_SCREEN,
    DELETE_CHAR,
    DELETE_CHARS,
    DELETE_LINE,
    DELETE_LINES,
    ERASE_CHARS,
    CLEAR_EOS,
    CLEAR_EOL,
    CLEAR_BOL,

    // insert mode
    SET_INSERT_MODE,
    INSERT_CHARS,

    // attributes
    SET_STANDOUT_MODE,
    SET_UNDERLINE_MODE,
    SET_BLINK_MODE,
    SET_BOLD_MODE,
    SET_INVISIBLEMODE,
    SET_REVERSE_MODE,
    EXIT_ATTRIBUTE_MODE,
    SET_CHARSET_MODE,

    // bell
    BELL,
    REVERSE_SCREEN,

    // cursor intensity
    CURSOR_VISIBILITY,

    // meta mode
    SET_META_MODE,

    // program initialization
    SAVE_SCREEN,
    RESTORE_SCREEN,

    // keypad activation
    SET_KEYPAD_MODE,
};

#endif

// vim: ts=4:sw=4:sts=4:expandtab
