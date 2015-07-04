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

    void Write(const uint8_t* data, int n) const;
    int Read(uint8_t* data, int max_sz) const;

private:
    int fd = 0;
};


#endif

// vim: ts=4:sw=4:sts=4:expandtab
