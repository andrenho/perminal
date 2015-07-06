#ifndef TERMINAL_H
#define TERMINAL_H

#include <cstdint>
#include <vector>
using namespace std;

#include "userevent.h"
#include "commands.h"
#include "matrix.h"
#include "charencoding.h"

class Capabilities;

class Terminal {
public:
    explicit Terminal(Capabilities const& cap);
    virtual ~Terminal() {}

    virtual int ParseUserEvent(UserEvent const& event, uint8_t* data) const;
    virtual Command ParsePluginOutput(uint8_t c, uint32_t pars[256]) const;

    virtual const string TERM() const;

private:
    mutable char buf[4];
    mutable int  buf_size = 0;
    Capabilities const& cap;
    CharEncoding ce;
};

#endif

// vim: ts=4:sw=4:sts=4:expandtab
