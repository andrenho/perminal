// Copyright 2015 André Wagner

#include "renderercurses.h"


namespace client {


RendererCurses::RendererCurses(Config const& config, ClientCharMatrix const& matrix)
    : Renderer(config, matrix)
{
}


void 
RendererCurses::Execute()
{
}


}  // namespace client

// vim: ts=4:sw=4:sts=4:expandtab
