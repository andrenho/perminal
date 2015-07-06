#include "matrix.h"

#include <cassert>
#include <cstdlib>

#include "debug.h"

Matrix::Matrix(int w, int h)
    : cursor(), w(w), h(h), cells(), dirty(), dirty_empty_screen()
{
    dirty.reserve(w*h);
    dirty_empty_screen.reserve(w*h);
    for(int y=0; y<h; ++y) {
        auto line = make_unique<vector<Cell>>();
        for(int x=0; x<w; ++x) {
            line->push_back(EmptyCell());
            dirty.push_back(P{x,y});
            dirty_empty_screen.push_back(P{x,y});
        }
        cells.push_back(move(line));
    }
    MoveCursor(0, 0);
}


void 
Matrix::Do(Command const& cmd, const uint32_t pars[256])
{
    switch(cmd) {
        case NONE: return;
        case REGULAR_INPUT: {
                char p[4] = { 
                    static_cast<char>(pars[0]), 
                    static_cast<char>(pars[1]), 
                    static_cast<char>(pars[2]), 
                    static_cast<char>(pars[3])
                };
                PrintChar(p);
                break;
            }

        case TAB:               if(cursor.x < (w-8)) { AdvanceX(8 - (cursor.x % 8)); } break;

        // local cursor movement
        case CARRIAGE_RETURN:   MoveCursor(0, cursor.y); break;
        case CURSOR_UP:         if(cursor.y > 0) { AdvanceY(-1); } break;
        case CURSOR_DOWN:       if(cursor.y < (h-1)) { AdvanceY(1); } break;
        case CURSOR_LEFT:       if(cursor.x > 0) { AdvanceX(-1); } break;
        case CURSOR_RIGHT:      if(cursor.x < (w-1)) { AdvanceX(1); } break;

        // classify (TODO)
        case BELL:              /* TODO */ printf("\a"); fflush(stdout); break;
        case CLEAR_EOL:         for(int x=cursor.x; x<w; ++x) { cells[cursor.y]->operator[](x) = EmptyCell(); } break;

        case IGNORE: abort(); // we shouldn't get here
        default: 
            abort();
    }
}


void
Matrix::Update()
{
    auto time = chrono::steady_clock::now() - last_blink;
    if(time > chrono::milliseconds(config.BlinkSpeed)) {
        blink_on = !blink_on;
        last_blink = chrono::steady_clock::now();
        // look for blinking cells
        for(int y=0; y<h; ++y) {
            for(int x=0; x<w; ++x) {
                if(cells[y]->at(x).attr.blink) {
                    dirty.push_back(P{x, y});
                }
            }
        }
    }
}


void 
Matrix::PrintChar(const char c[4])
{
    cells[cursor.y]->at(cursor.x).c[0] = c[0];
    cells[cursor.y]->at(cursor.x).c[1] = c[1];
    cells[cursor.y]->at(cursor.x).c[2] = c[2];
    cells[cursor.y]->at(cursor.x).c[3] = c[3];
    cells[cursor.y]->at(cursor.x).attr = CurrentAttr;
    dirty.push_back(cursor);

    AdvanceX(1);
}


vector<P>
Matrix::Dirty() const
{
    vector<P> cp = dirty;
    dirty.clear();
    redraw_screen = false;
    return cp;
}


void 
Matrix::MoveCursor(int x, int y)
{
    dirty.push_back(cursor);
    cursor.x = x;
    cursor.y = y;
    dirty.push_back(cursor);
}


void
Matrix::AdvanceX(int n)
{
    if(cursor.x+n >= w) {
        MoveCursor(0, cursor.y);
        AdvanceY(1);
    } else {
        MoveCursor(cursor.x+n, cursor.y);
    }
}


void
Matrix::AdvanceY(int n)
{
    int lines_to_scroll = (cursor.y + n) - h + 1;
    if(lines_to_scroll > 0) {
        ScrollLines(lines_to_scroll);
    }
    MoveCursor(cursor.x, min(cursor.y+n, h-1));
}


void
Matrix::ScrollLines(int n)
{
    if(n > 0) {
        // move lines
        for(int y=n; y<h; ++y) {
            cells[y-n] = move(cells.at(y));
        }
        
        // clear lines
        for(int y=h-n; y<h; ++y) {
            auto line = new vector<Cell>();
            for(int x=0; x<w; ++x) {
                line->push_back(EmptyCell());
            }
            cells[y].reset(line);
        }
    }

    RedrawScreen();
}


void 
Matrix::Resize(int nw, int nh)
{
    D("Resizing matrix from %dx%d to %dx%d\n", w, h, nw, nh);

    // adjust rows
    if(nh < h) {
        if(cursor.y >= nh) {
            ScrollLines(cursor.y - nh - 1);
            MoveCursor(cursor.x, cursor.y - (cursor.y - nh)-1);
        }
        for(int y=nh; y<h; ++y) {
            cells.pop_back();
        }
    } else if(nh > h) {
        for(int y=h; y<nh; ++y) {
            auto line = make_unique<vector<Cell>>();
            for(int x=0; x<w; ++x) {
                line->push_back(EmptyCell());
                dirty.push_back(P{x,y});
            }
            cells.push_back(move(line));
        }
    }
    ASSERT(cells.size() == static_cast<size_t>(nh));
    h = nh;

    // adjust columns
    for(int y=0; y<h; ++y) {
        if(nw<w) {
            for(int x=nw; x<w; ++x) {
                cells[y]->pop_back();
            }
        } else if(nw>w) {
            for(int x=w; x<nw; ++x) {
                cells[y]->push_back(EmptyCell());
                dirty.push_back(P{x,y});
            }
        }
        ASSERT(cells[y]->size() == static_cast<size_t>(nw));
    }
    w = nw;

    // rebuild dirty_empty_screen
    dirty_empty_screen.clear();
    for(int y=0; y<h; ++y) {
        for(int x=0; x<w; ++x) {
            dirty_empty_screen.push_back(P{x,y});
        }
    }
}


void 
Matrix::RedrawScreen()
{
    if(redraw_screen) {
        return;
    }

    dirty = dirty_empty_screen;
    redraw_screen = true;
}


// vim: ts=4:sw=4:sts=4:expandtab
