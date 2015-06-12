// Copyright 2015 André Wagner

#include <ncurses.h>

#include "renderercurses.h"
#include "clientcharmatrix.h"


namespace client {


RendererCurses::RendererCurses(Config const& config, ClientCharMatrix& matrix)
    : Renderer(config, matrix)
{
    initscr();
    cbreak();
    noecho();
    keypad(stdscr, TRUE);
    timeout(-1);
    curs_set(0);
    refresh();
}


RendererCurses::~RendererCurses()
{
    endwin();
}


void 
RendererCurses::Execute()
{
    while(true) {
        UpdateFromMatrix();

        int ch = getch();
        switch(ch) {
            case KEY_F(12):
                return;
        }
    }
}


void 
RendererCurses::UpdateFromMatrix()
{
    for(auto const& u : matrix.Updates()) {
        mvaddch(u.x, u.y, u.ch);
    }
    matrix.ClearUpdates();
    refresh();
}


}  // namespace client

// vim: ts=4:sw=4:sts=4:expandtab
