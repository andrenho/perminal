#ifndef DUMB_H
#define DUMB_H

#include "capabilities.h"

class Dumb : public Capabilities {
public:
    Dumb() : Capabilities({}, {}) {}

    const string TERM() const override { return "dumb"; }

private:
    char EnterCapModeChar() const override { return 0; }
    int MaxCapSize() const override { return 0; }
};

#endif

// vim: ts=4:sw=4:sts=4:expandtab
