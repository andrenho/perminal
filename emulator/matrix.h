#ifndef MATRIX_H
#define MATRIX_H

#include <chrono>
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
    void Update();

    vector<P> Dirty() const;

    int Width() const { return w; }
    int Height() const { return h; }
    Cell const& Cells(int x, int y) const { return cells.at(P{ x, y }); }

    bool Blinking() const { return !blink_on; }

private:
    const Attributes DEFAULT_ATTR = { 
        false,false,false,false,false,false,false,false,false,false, config.DefaultBGColor, config.DefaultFGColor,
    };

public:
    Attributes CurrentAttr = DEFAULT_ATTR;
    Cursor cursor;

private:
    int w, h;
    map<P, Cell> cells;
    mutable vector<P> dirty;

    chrono::time_point<chrono::steady_clock> last_blink = chrono::steady_clock::now();

    bool blink_on = true;
};

#endif

// vim: ts=4:sw=4:sts=4:expandtab
