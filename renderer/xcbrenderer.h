#ifndef XCBRENDERER_H
#define XCBREDNERER_H

#include <functional>
using namespace std;

#include "renderer.h"
#include "font.h"
#include "chars.h"

class XcbRenderer : public Renderer {
public:
    explicit XcbRenderer(Font const& font);
    ~XcbRenderer();

    vector<UserEvent> GetEvents() const;
    void Update(Matrix const& matrix) const;

    bool Running() const { return active; }

private:
    void RedrawBorder() const;
    uint32_t DoWithColor(Color const& c, function<void(uint32_t)> f) const;

    Font const& font;
    mutable bool active = true;
    int win_w = 800, win_h = 600;

    // xcb data
    struct xcb_connection_t *c;
    uint32_t window;
    uint32_t gc = 0;
    uint32_t colormap = 0;

    XcbRenderer(XcbRenderer const&) = delete;
    XcbRenderer(XcbRenderer&&) = delete;
    XcbRenderer& operator=(XcbRenderer const&) = delete;
    XcbRenderer& operator=(XcbRenderer&&) = delete;
};

#endif

// vim: ts=4:sw=4:sts=4:expandtab
