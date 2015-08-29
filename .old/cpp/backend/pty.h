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
    explicit PTY(string const& term);
    ~PTY();

    void Write(const uint8_t* data, int n) const;
    int Read(uint8_t* data, int max_sz) const;
    void Resize(int w, int h) const;

private:
    void PrintMOTD() const;

    int fd = 0;
};


#endif

// vim: ts=4:sw=4:sts=4:expandtab
