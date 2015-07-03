#include "matrix.h"

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
    PrintChar("é");
}


void 
Matrix::PrintChar(const char c[4])
{
    cells[cursor].c[0] = c[0];
    cells[cursor].c[1] = c[1];
    cells[cursor].c[2] = c[2];
    cells[cursor].c[3] = c[3];
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
