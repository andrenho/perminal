#include "dumbterminal.h"

#include <cassert>
#include <cstring>

#include "debug.h"

DumbTerminal::DumbTerminal()
    : ce("utf-8", "latin1")
{
}


int 
DumbTerminal::ParseUserEvent(UserEvent const& event, uint8_t* data) const
{
    if(event.type == KEYPRESS) {
        int n = strlen(reinterpret_cast<const char*>(event.chr));
        assert(n<4);
        for(int i=0; i<n; ++i) {
            data[i] = event.chr[i];
        }
        return n;
    }
    return 0;
}


Command 
DumbTerminal::ParsePluginOutput(uint8_t c, uint32_t pars[256]) const
{
    // rotate buffer
    assert(buf_size < 4);  // TODO
    buf[buf_size++] = c;
    if(ce.IsComplete(buf, buf_size)) {
        if(buf[0] < 32) {
            buf_size = 0;
        }
        switch(buf[0]) {
            case 7:
                return BELL;
            case 8:
                return BACKSPACE;
            case 9:
                return TAB;
            case 10:
                return LINE_FEED;
            case 13:
                return CARRIAGE_RETURN;
            case 27:
                // TODO
                break;
            default:
                // copy buffer to parameters
                pars[0] = buf[0]; pars[1] = buf[1]; pars[2] = buf[2]; pars[3] = buf[3];
                pars[buf_size] = 0;
                buf_size = 0;
                return REGULAR_INPUT;
        }
    }
    return NONE;
}


// vim: ts=4:sw=4:sts=4:expandtab
