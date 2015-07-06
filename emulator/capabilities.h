#ifndef CAPABILITIES_H
#define CAPABILITIES_H

#include <cstdint>
#include <map>
#include <sstream>
#include <string>
#include <vector>
#include <utility>
using namespace std;

#include "commands.h"
#include "userevent.h"


class Capabilities {
public:
    Capabilities(map<UserEvent, vector<uint8_t>> const& user_events, map<string, Command> const& capabilities);
    virtual ~Capabilities() {}
    
    virtual int ParseUserEvent(UserEvent const& event, uint8_t* data) const;
    virtual Command ParseCapability(char c, uint32_t pars[256]) const;

    virtual const string TERM() const = 0;

protected:
    virtual char EnterCapModeChar() const = 0;
    virtual int MaxCapSize() const = 0;
    string ParseCap(string const& cap, uint32_t pars[256]) const;

    map<UserEvent, vector<uint8_t>> user_events;
    map<string, Command> capabilities;

    mutable bool cap_mode = false;
    mutable stringstream current_cap;
};

#endif

// vim: ts=4:sw=4:sts=4:expandtab
