#include "xcbrenderer.h"

#include <xcb/xcb.h>

XcbRenderer::XcbRenderer(Font const& font) 
    : font(font), c(xcb_connect(nullptr, nullptr))
{
    if(!c) {
        throw "could not connect to a X server";
    }
}


XcbRenderer::~XcbRenderer()
{
}


vector<UserEvent> 
XcbRenderer::GetEvents() const 
{ 
    return {}; 
}
    

void 
XcbRenderer::Update(Matrix const& matrix) const 
{ 
    (void) matrix;
}

// vim: ts=4:sw=4:sts=4:expandtab
