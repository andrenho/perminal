#include "matrix.h"


#include <cassert>

#include "debug.h"

Matrix::Matrix(int w, int h)
    : cursor(), w(w), h(h), cells(), dirty()
{
    for(int x=0; x<w; ++x) {
        for(int y=0; y<h; ++y) {
            cells[P{x,y}] = { { ' ', 0, 0, 0 }, DEFAULT_ATTR };
            dirty.push_back(P{x,y});
        }
    }

    PrintChar("a");
    CurrentAttr.blink = true;
    PrintChar("é");
}


void
Matrix::Update()
{
    auto time = chrono::steady_clock::now() - last_blink;
    if(time > chrono::milliseconds(config.BlinkSpeed)) {
        blink_on = !blink_on;
        last_blink = chrono::steady_clock::now();
        // look for blinking cells
        for(auto& kv: cells) {
            if(kv.second.attr.blink) {
                dirty.push_back(kv.first);
            }
        }
    }
}


void 
Matrix::PrintChar(const char c[4])
{
    cells[cursor].c[0] = c[0];
    cells[cursor].c[1] = c[1];
    cells[cursor].c[2] = c[2];
    cells[cursor].c[3] = c[3];
    cells[cursor].attr = CurrentAttr;
    dirty.push_back(cursor);

    // TODO - advance cursor
    ++cursor.x;
}


vector<P>
Matrix::Dirty() const
{
    vector<P> cp = dirty;
    dirty.clear();
    cp.push_back(cursor);
    return cp;
}



// vim: ts=4:sw=4:sts=4:expandtab
