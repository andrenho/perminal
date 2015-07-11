#include "capabilities.h"

#include <cstring>
#include <algorithm>

#include "debug.h"

Capabilities::Capabilities(map<UserEvent, string> const& user_events, 
                           map<string, Command> const& capabilities)
    : user_events(user_events), capabilities(capabilities), current_cap()
{
}


int 
Capabilities::ParseUserEvent(UserEvent const& event, uint8_t* data) const
{
    auto it = user_events.find(event);
    if(it != user_events.end()) {
        int i = 0;
        for(char const& c: it->second) {
            data[i++] = c;
        }
        return i;
    }
    return 0;
}


Command 
Capabilities::ParseCapability(char c, uint32_t pars[256]) const
{
    if(!cap_mode) {
        if(c != 0 and c == EnterCapModeChar()) {
            cap_mode = true;
            current_cap.str("");
        } 
    }

    if(cap_mode) { 
        C(c, true, true);

        current_cap.put(c);

        // check sanity
        if(current_cap.str().length() > MaxCapSize()) {
            D("Capability descriptor too long -- giving up");
            int i=0;
            for(char const& c: current_cap.str()) {
                //C(c, true, true);
                pars[i++] = c;
            }
            pars[i] = 0;
            cap_mode = false;
            current_cap.str("");
            return UNWIND;
        }

        // verify if capability matches
        uint32_t p[256];
        string cap = ParseCap(current_cap.str(), p);
        auto it = capabilities.find(cap);
        if(it != capabilities.end()) {
            cap_mode = false;
            current_cap.str("");
            memcpy(pars, p, 256 * sizeof(uint32_t));
            return it->second;
        }

        return NONE;
    }

    return IGNORE;
}


string 
Capabilities::ParseCap(string const& cap, uint32_t pars[256]) const
{
    return cap;  // TODO
}


// vim: ts=4:sw=4:sts=4:expandtab
