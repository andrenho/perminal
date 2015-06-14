// Copyright 2015 André Wagner

#include <ncurses.h>

#include "renderercurses.h"


RendererCurses::RendererCurses(Config const& config)
    : Renderer(config)
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
RendererCurses::SetChar(int x, int y, char ch)
{
    mvaddch(y, x, ch);
}


void
RendererCurses::Refresh()
{
    refresh();
}


// vim: ts=4:sw=4:sts=4:expandtab
