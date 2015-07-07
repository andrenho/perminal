#ifndef TERMINAL_H
#define TERMINAL_H

#include <cstdint>
#include <map>
#include <sstream>
#include <vector>
using namespace std;

#include "userevent.h"
#include "commands.h"
#include "matrix.h"
#include "charencoding.h"

class Terminal {
public:
    Terminal();
    virtual ~Terminal() {}

    int ParseUserEvent(UserEvent const& event, uint8_t* data) const;
    Command ParsePluginOutput(uint8_t c, uint32_t pars[256]) const;

    const string TERM() const { return "perminal"; }

private:
    Command ParseCapability(uint32_t pars[256]) const;

    mutable char buf[4];
    mutable int  buf_size = 0;
    CharEncoding ce;
    mutable bool cap_on = false;
    mutable stringstream cap;

    map<string, Command> capabilities = {
        // local cursor movement
        { "@cuf1|", CURSOR_RIGHT },
        { "@cuu1|", CURSOR_UP },
        { "@home|", CURSOR_HOME },
        // others (TODO)
        { "@el|", CLEAR_EOL },
    };

    map<UserEvent, string> user_events = {
        { UserEvent(KEYPRESS_SP, UP),    "\eOA" },
        { UserEvent(KEYPRESS_SP, DOWN),  "\eOB" },
        { UserEvent(KEYPRESS_SP, RIGHT), "\eOC" },
        { UserEvent(KEYPRESS_SP, LEFT),  "\eOD" },
    };
};

#endif

// vim: ts=4:sw=4:sts=4:expandtab
