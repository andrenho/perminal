#include "terminal.h"

#include <cassert>
#include <cstring>

#include "capabilities.h"
#include "debug.h"

Terminal::Terminal()
    : ce("utf-8", "latin1"), cap()
{
}


int 
Terminal::ParseUserEvent(UserEvent const& event, uint8_t* data) const
{
    int n = 0;

    if(event.type == KEYPRESS_CH) {
        n = strlen(reinterpret_cast<const char*>(event.chr));  // get UTF-8 from key pressed
    }

    if(n > 0) {
        // fill out `data` with the bytes that will be sent back to the plugin
        assert(n<4);
        for(int i=0; i<n; ++i) {
            data[i] = event.chr[i];
        }
    } else {
        // find in capabilities
        auto it = user_events.find(event);
        if(it != user_events.end()) {
            for(char const& c: it->second) {
                data[n++] = c;
            }
        }
    }

    return n;
}


Command 
Terminal::ParsePluginOutput(uint8_t c, uint32_t pars[256]) const
{
    if(!cap_on) {

        // rotate buffer
        assert(buf_size < 4);  // TODO
        buf[buf_size++] = c;
        if(ce.IsComplete(buf, buf_size)) {

            // second, do a simple parse
            C(c, true, c == 27);
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
                case 27:
                    buf_size = 0; cap_on = true; cap.str("\e"); return NONE;
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
    
    } else { // if(cap_on)
    
        C(c, true, true);
        cap.put(c);

        if(c == '|') {
            Command cmd = ParseCapability(pars);
            if(cmd != NONE) {
                cap_on = false;
                cap.str("");
            }
            return cmd;
        }

    }

    return NONE;
}


Command
Terminal::ParseCapability(uint32_t pars[256]) const
{
    auto it = capabilities.find(cap.str());  // TODO - parse parameters
    if(it != capabilities.end()) {
        return it->second;
    } else if(cap.str().length() > 20) {   // len(cap) > 20
        int i=0;
        for(char const& c: cap.str()) {
            //C(c, true, true);
            pars[i++] = c;
        }
        pars[i] = 0;
        return UNWIND;
    } else {   // cap not found
        return NONE;
    }
}


// vim: ts=4:sw=4:sts=4:expandtab
