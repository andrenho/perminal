#ifndef MATRIX_H
#define MATRIX_H

#include <map>
#include <vector>
using namespace std;

#include "chars.h"
#include "config.h"
#include "cursor.h"

class Matrix {
public:
    Matrix(int w, int h);

    void PrintChar(const char c[4]);

    vector<P> Dirty() const;

    int Width() const { return w; }
    int Height() const { return h; }
    Cell const& Cells(int x, int y) const { return cells.at(P{ x, y }); }

    Attributes CurrentAttr = DEFAULT_ATTR;

    Cursor cursor;

private:
    int w, h;
    map<P, Cell> cells;
    mutable vector<P> dirty;

    bool blink_on = true;

    const Attributes DEFAULT_ATTR = { 
        false,false,false,false,false,false,false,false,false,false, config.DefaultBGColor, config.DefaultFGColor,
    };
};

#endif

// vim: ts=4:sw=4:sts=4:expandtab
