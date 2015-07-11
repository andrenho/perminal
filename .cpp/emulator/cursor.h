#ifndef CURSOR_H
#define CURSOR_H

#include "config.h"
#include "chars.h"

struct P {
    int x, y;

    inline bool operator<(P const& other) const {
        return ((x<<16) + y) < ((other.x<<16) + other.y);
    }

    inline bool operator==(P const& other) const {
        return other.x == x && other.y == y;
    }
};

class Cursor {
public:
    Cursor() : x(0), y(0) {}

    operator P() const { return P{x, y}; }

    Color color() const;

    int x, y;
    enum { INVISIBLE, VISIBLE, VERY_VISIBLE } intensity = VISIBLE;
};

#endif

// vim: ts=4:sw=4:sts=4:expandtab
