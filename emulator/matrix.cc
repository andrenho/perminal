#include "matrix.h"

Matrix::Matrix(int w, int h)
    : w(w), h(h), cells(), dirty()
{
    for(int x=0; x<w; ++x) {
        for(int y=0; y<h; ++y) {
            cells[P{x,y}] = { { ' ', 0, 0, 0 }, DEFAULT_ATTR };
            dirty.push_back(P{x,y});
        }
    }
    PrintChar("A");
    PrintChar("n");
    PrintChar("d");
    PrintChar("r");
    char s[] { 0xe0, 0x09, 0x0 };
    PrintChar(s);
}


vector<P>
Matrix::Dirty() const
{
    vector<P> cp = dirty;
    dirty.clear();
    return cp;
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


// vim: ts=4:sw=4:sts=4:expandtab