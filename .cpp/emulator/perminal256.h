#ifndef PERMINAL256_H
#define PERMINAL256_H

#include "capabilities.h"

class Perminal256 : public Capabilities {
public:
    Perminal256() : Capabilities({
        //
        // keys
        // 
        { UserEvent(KEYPRESS_SP, UP),    "\eOA" },
        { UserEvent(KEYPRESS_SP, DOWN),  "\eOB" },
        { UserEvent(KEYPRESS_SP, RIGHT), "\eOC" },
        { UserEvent(KEYPRESS_SP, LEFT),  "\eOD" },
    }, {
        //
        // capabilities
        //
        
        // local cursor movement
        { "\e[C", CURSOR_RIGHT },
        { "\e[A", CURSOR_UP },
        { "\e[H", CURSOR_HOME },
        
        { "\e[K", CLEAR_EOL },
    }) {}

    const string TERM() const override { return "perminal-256color"; }

private:
    char EnterCapModeChar() const override { return 27; }
    int MaxCapSize() const override { return 20; }
};

#endif

// vim: ts=4:sw=4:sts=4:expandtab
