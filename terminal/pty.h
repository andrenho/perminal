#ifndef PTY_H
#define PTY_H

#include <cstdint>
#include <string>
#include <stdexcept>
#include <vector>
using namespace std;

#include "plugin.h"

class PTY : public Plugin {
public:
    PTY(string const& term);
    ~PTY();

    void Write(vector<uint8_t> const& data) const;
    vector<uint8_t> Read() const;

private:
    int fd = 0;
};


//
// exceptions
//
struct PTYException : public runtime_error {
    explicit PTYException(string const& msg) : runtime_error(msg) {}
};

#endif

// vim: ts=4:sw=4:sts=4:expandtab
