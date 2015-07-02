#ifndef TERMINAL_H
#define TERMINAL_H

#include <cstdint>
#include <vector>
using namespace std;

#include "userevent.h"
#include "matrix.h"

class Terminal {
public:
    Terminal() : matrix(80, 25) {}

    bool Alive() const { return true; }
    vector<uint8_t> ParseEvent(UserEvent const& event) const { (void) event; return {}; }
    Matrix const& ParseData(vector<uint8_t> const& data) const { (void) data; return matrix; }

private:
    Matrix matrix;
};

#endif

// vim: ts=4:sw=4:sts=4:expandtab
