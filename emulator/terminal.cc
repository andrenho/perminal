#include "terminal.h"

#include <cassert>
#include <cstring>

#include "capabilities.h"
#include "debug.h"

Terminal::Terminal(Capabilities const& cap)
    : cap(cap), ce("utf-8", "latin1")
{
}


int 
Terminal::ParseUserEvent(UserEvent const& event, uint8_t* data) const
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
Terminal::ParsePluginOutput(uint8_t c, uint32_t pars[256]) const
{
    // rotate buffer
    assert(buf_size < 4);  // TODO
    buf[buf_size++] = c;
    if(ce.IsComplete(buf, buf_size)) {
        C(c, true);
        switch(buf[0]) {
            case 7:
                buf_size = 0;
                return BELL;
            case 8:
                buf_size = 0;
                return BACKSPACE;
            case 9:
                buf_size = 0;
                return TAB;
            case 10:
                buf_size = 0;
                return LINE_FEED;
            case 13:
                buf_size = 0;
                return CARRIAGE_RETURN;
            case 27:
                pars[0] = 27;   // useful for testing
                buf_size = 0;
                return REGULAR_INPUT;
            default:
                // copy buffer to parameters
                pars[0] = buf[0]; pars[1] = buf[1]; pars[2] = buf[2]; pars[3] = buf[3];
                pars[buf_size] = 0;
                buf_size = 0;
                return REGULAR_INPUT;
        }
    } else {
        C(c, false);
    }
    return NONE;
}


// vim: ts=4:sw=4:sts=4:expandtab
