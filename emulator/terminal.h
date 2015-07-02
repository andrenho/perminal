#ifndef TERMINAL_H
#define TERMINAL_H

#include <cstdint>
#include <vector>
using namespace std;

#include "userevent.h"
#include "matrix.h"

class Terminal {
public:
    Terminal(Matrix const& matrix) : matrix(matrix) {}

    bool Alive() const { return true; }
    vector<uint8_t> ParseEvent(UserEvent const& event) const { (void) event; return {}; }
    void ParseData(vector<uint8_t> const& data) const { (void) data; }

private:
    Matrix const& matrix;
};

#endif

// vim: ts=4:sw=4:sts=4:expandtab
