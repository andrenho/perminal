#ifndef XCBRENDERER_H
#define XCBREDNERER_H

#include "renderer.h"
#include "font.h"

class XcbRenderer : public Renderer {
public:
    explicit XcbRenderer(Font const& font);
    ~XcbRenderer();

    vector<UserEvent> GetEvents() const;
    void Update(Matrix const& matrix) const;

    bool Running() const { return active; }

private:
    Font const& font;
    bool active = true;
    struct xcb_connection_t *c;

    XcbRenderer(XcbRenderer const&) = delete;
    XcbRenderer(XcbRenderer&&) = delete;
    XcbRenderer& operator=(XcbRenderer const&) = delete;
    XcbRenderer& operator=(XcbRenderer&&) = delete;
};

#endif

// vim: ts=4:sw=4:sts=4:expandtab
