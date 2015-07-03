#ifndef MATRIX_H
#define MATRIX_H

#include <map>
#include <vector>
using namespace std;

#include "chars.h"
#include "config.h"

struct P {
    int x, y;

    inline bool operator<(P const& other) const {
        return ((x<<16) + y) < ((other.x<<16) + other.y);
    }
};

enum ECursorIntensity { INVISIBLE, VISIBLE, VERY_VISIBLE };

class Matrix {
public:
    Matrix(int w, int h);

    void PrintChar(const char c[4]);

    inline void Blink() { blink_on = !blink_on; }

    vector<P> Dirty() const;
    inline void AddToDirty(P p) const { dirty.push_back(p); }

    int Width() const { return w; }
    int Height() const { return h; }
    Cell const& Cells(int x, int y) const { return cells.at(P{ x, y }); }
    P Cursor() const { return cursor; }
    ECursorIntensity CursorIntensity() const;

    Attributes CurrentAttr = DEFAULT_ATTR;

private:
    int w, h;
    map<P, Cell> cells;
    mutable vector<P> dirty;

    P cursor = { 0, 0 };
    ECursorIntensity cursor_intensity = VISIBLE;
    bool blink_on = true;

    const Attributes DEFAULT_ATTR = { 
        false,false,false,false,false,false,false,false,false,false, config.DefaultBGColor, config.DefaultFGColor,
    };
};

#endif

// vim: ts=4:sw=4:sts=4:expandtab
