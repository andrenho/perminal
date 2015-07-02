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

class Matrix {
public:
    Matrix(int w, int h);

    void PrintChar(const char c[4]);

    int Width() const { return w; }
    int Height() const { return h; }
    Cell const& Cells(int x, int y) const { return cells.at(P{ x, y }); }
    vector<P> Dirty() const;
    P Cursor() const { return cursor; }

    Attributes CurrentAttr = DEFAULT_ATTR;

private:
    int w, h;
    map<P, Cell> cells;
    mutable vector<P> dirty;

    P cursor = { 0, 0 };

    const Attributes DEFAULT_ATTR = { 
        false,false,false,false,false,false,false,false,false,false, config.DefaultBGColor, config.DefaultFGColor,
    };
};

#endif

// vim: ts=4:sw=4:sts=4:expandtab
