#ifndef PLUGIN_H
#define PLUGIN_H

#include <vector>
using namespace std;

class Plugin {
public:
    virtual ~Plugin() {}
    virtual void Write(vector<uint8_t> const& data) const = 0;
    virtual vector<uint8_t> Read() const = 0;
};

#endif

// vim: ts=4:sw=4:sts=4:expandtab
