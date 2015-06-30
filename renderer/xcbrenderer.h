#ifndef XCBRENDERER_H
#define XCBREDNERER_H

#include "renderer.h"
#include "font.h"

class XcbRenderer : public Renderer {
public:
    explicit XcbRenderer(Font const& font) : font(font) {}
    bool Running() const { return true; }
    vector<UserEvent> GetEvents() const { return {}; }
    void Update(Matrix const& matrix) const { (void) matrix; }

private:
    Font const& font;
};

#endif

// vim: ts=4:sw=4:sts=4:expandtab
