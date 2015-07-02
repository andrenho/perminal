#ifndef PTY_H
#define PTY_H

#include <vector>
using namespace std;

#include "plugin.h"

class PTY : public Plugin {
public:
    PTY() {}

    void Write(vector<uint8_t> const& data) const { (void) data; }
    vector<uint8_t> Read() const { return {}; }
};

#endif

// vim: ts=4:sw=4:sts=4:expandtab
