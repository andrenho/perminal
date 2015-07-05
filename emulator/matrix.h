#ifndef MATRIX_H
#define MATRIX_H

#include <chrono>
#include <memory>
#include <vector>
using namespace std;

#include "chars.h"
#include "config.h"
#include "cursor.h"
#include "commands.h"

class Matrix {
public:
    Matrix(int w, int h);

    void Do(Command const& cmd, const uint32_t pars[256]);
    void Update();

    vector<P> Dirty() const;

    inline int Width() const { return w; }
    inline int Height() const { return h; }
    inline Cell const& Cells(int x, int y) const { return cells[y]->at(x); }
    inline bool Blinking() const { return !blink_on; }

    void Resize(int w, int h);

    Cursor cursor;

private:
    void PrintChar(const char c[4]);
    void MoveCursor(int x, int y);
    void AdvanceX(int n);
    void AdvanceY(int n);
    void ScrollLines(int n);

    void RedrawScreen();

    const Attributes DEFAULT_ATTR = { 
        false,false,false,false,false,false,false,false,false,false, config.DefaultBGColor, config.DefaultFGColor,
    };

    Attributes CurrentAttr = DEFAULT_ATTR;

    Cell EmptyCell() const { return { { ' ', 0, 0, 0 }, CurrentAttr }; }

    int w, h;
    vector<unique_ptr<vector<Cell>>> cells;
    mutable vector<P> dirty;
    vector<P> dirty_empty_screen;
    mutable bool redraw_screen = false;

    chrono::time_point<chrono::steady_clock> last_blink = chrono::steady_clock::now();

    bool blink_on = true;
};

#endif

// vim: ts=4:sw=4:sts=4:expandtab
