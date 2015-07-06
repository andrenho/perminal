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
    int n = 0;

    if((n = cap.ParseUserEvent(event, data)) == 0) {   // parse from capability
        
        // capability not found, do a simple parse
        if(event.type == KEYPRESS_CH) {
            n = strlen(reinterpret_cast<const char*>(event.chr));  // get UTF-8 from key pressed
        }

        // fill out `data` with the bytes that will be sent back to the plugin
        if(n > 0) {
            assert(n<4);
            for(int i=0; i<n; ++i) {
                data[i] = event.chr[i];
            }
        }
    }

    return n;
}


Command 
Terminal::ParsePluginOutput(uint8_t c, uint32_t pars[256]) const
{
    // rotate buffer
    assert(buf_size < 4);  // TODO
    buf[buf_size++] = c;
    if(ce.IsComplete(buf, buf_size)) {

        // first, send key to cap
        if(c < 128) {
            Command cmd = cap.ParseCapability(static_cast<char>(c), pars);
            if(cmd != IGNORE) {
                buf_size = 0;
                return cmd;
            }
        }

        // second, do a simple parse
        C(c, true);
        switch(buf[0]) {
            case 7:
                buf_size = 0; return BELL;
            case 8:
                buf_size = 0; return CURSOR_LEFT;
            case 9:
                buf_size = 0; return TAB;
            case 10:
                buf_size = 0; return CURSOR_DOWN;
            case 13:
                buf_size = 0; return CARRIAGE_RETURN;
            default:
                // copy buffer to parameters
                pars[0] = buf[0]; pars[1] = buf[1]; pars[2] = buf[2]; pars[3] = buf[3];
                pars[buf_size] = 0;
                buf_size = 0;
                return REGULAR_INPUT;
        }
    } else {
        // if we got here, is because this char is just part of a longer UTF-8 sequence
        C(c, false);
    }
    return NONE;
}


const string 
Terminal::TERM() const
{
    return cap.TERM();
}

// vim: ts=4:sw=4:sts=4:expandtab
