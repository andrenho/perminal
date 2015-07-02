#ifndef XCBRENDERER_H
#define XCBRENDERER_H

#include <functional>
#include <map>
#include <memory>
using namespace std;

#include "renderer.h"
#include "font.h"
#include "chars.h"
#include "xkb_keyboard.h"


class XcbRenderer : public Renderer {
public:
    explicit XcbRenderer(Font const& font);
    ~XcbRenderer();

    UserEvent GetEvent() const;
    void Update(Matrix const& matrix) const;

    bool Running() const { return active; }

private:
    void RedrawBorder() const;
    void DrawChar(int x, int y, const char c[4], Attributes const& attr) const;

    uint32_t GetColor(Color const& color) const;
    uint32_t GetCharPixmap(const char c[4], Attributes const& attr) const;
    void SetGCForeground(Color const& color) const;

    // color memoization
    mutable map<Color, 
                unique_ptr<struct xcb_alloc_color_reply_t, 
                           function<void(struct xcb_alloc_color_reply_t*)>>
               > colors = {};
    mutable map<Cell, uint32_t> ch_pixmaps = {};
        
    // xcb data
    struct xcb_connection_t *c;
    uint32_t window;
    uint32_t gc = 0;
    uint32_t colormap = 0;
    uint8_t  depth = 0;

    // class data
    Font const& font;
    mutable bool active = true;
    uint16_t win_w = 800, win_h = 600;
    XkbKeyboard keyboard;

    XcbRenderer(XcbRenderer const&) = delete;
    XcbRenderer(XcbRenderer&&) = delete;
    XcbRenderer& operator=(XcbRenderer const&) = delete;
    XcbRenderer& operator=(XcbRenderer&&) = delete;
};


#endif

// vim: ts=4:sw=4:sts=4:expandtab
