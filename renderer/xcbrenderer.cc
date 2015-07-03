#include "xcbrenderer.h"

#include <xcb/xcb.h>

#include <cassert>
#include <cstring>

#include "config.h"
#include "cursor.h"
#include "debug.h"

XcbRenderer::XcbRenderer(Matrix const& matrix, Font const& font) 
    : Renderer(matrix), c(xcb_connect(nullptr, nullptr)), window(xcb_generate_id(c)), 
      font(font), win_w(matrix.Width() * font.CharWidth()), win_h(matrix.Height() * font.CharHeight()),
      keyboard(c)
{
    // check open connection
    if(!c) {
        throw RendererInitException("could not connect to a X server");
    }
    D("Connected to a X server.");

    // get screen information
    int screen_nbr;
    xcb_screen_t* screen = nullptr;
    xcb_screen_iterator_t iter = xcb_setup_roots_iterator(xcb_get_setup(c));
    for(; iter.rem; --screen_nbr, xcb_screen_next(&iter)) {
        screen = iter.data;
        break;
    }
    depth = screen->root_depth;
    D("Info about screen %d", screen->root);   
    D("  width:  %d", screen->width_in_pixels);
    D("  height: %d", screen->height_in_pixels);
    D("  white px: 0x%x", screen->white_pixel);
    D("  black px: 0x%x", screen->black_pixel);

    // register for events
    uint32_t mask = XCB_CW_BACK_PIXEL | XCB_CW_EVENT_MASK;
    uint32_t values[2] = {
        screen->white_pixel,
        XCB_EVENT_MASK_STRUCTURE_NOTIFY 
            | XCB_EVENT_MASK_EXPOSURE
            | XCB_EVENT_MASK_KEY_PRESS
    };

    // create colormap & draw border
    colormap = screen->default_colormap;

    // create window
    xcb_create_window(c, 
            XCB_COPY_FROM_PARENT,                       // depth
            window,                                     // id
            screen->root,                               // parent
            0, 0,                                       // position
            win_w + (config.BorderSize.LeftRight * 2),  // window width
            win_h + (config.BorderSize.TopBottom * 2),  // window height
            0,                                          // border width
            XCB_WINDOW_CLASS_INPUT_OUTPUT,              // class
            screen->root_visual,                        // visual
            mask, values);                              // masks
    xcb_map_window(c, window);
    xcb_flush(c);

    // initial image (makes it look like it's fast)
    xcb_generic_event_t* e;
    do { e = xcb_wait_for_event(c); } while((e->response_type & ~0x80) != XCB_MAP_NOTIFY);
    RedrawBorder();
    xcb_flush(c);

    D("Window created.");

    // create GC
    gc = xcb_generate_id(c);
    uint32_t value[2] = { screen->black_pixel, 0, };
    xcb_create_gc(c, gc, window, XCB_GC_FOREGROUND | XCB_GC_GRAPHICS_EXPOSURES, value);
    D("Created GC.");
}


XcbRenderer::~XcbRenderer()
{
    xcb_disconnect(c);
}


void 
XcbRenderer::Update() const 
{ 
    // draw chars
    for(auto const& p: matrix.Dirty()) {
        auto const& cell = matrix.Cells(p.x, p.y);
        DrawChar(p.x, p.y, cell.c, cell.attr);
    }
    xcb_flush(c);
}


UserEvent 
XcbRenderer::GetEvent() const 
{ 
    xcb_generic_event_t* e = xcb_poll_for_event(c);
    if(!e) { return { { NOTHING } }; }
    switch(e->response_type & ~0x80) {
    case XCB_EXPOSE: {
        xcb_expose_event_t *ex = reinterpret_cast<xcb_expose_event_t *>(e);
        D("Expose event detected (%d %d %d %d) count %d", ex->x, ex->y, ex->width, ex->height, ex->count);
        RedrawBorder();
        int x1 = ex->x / font.CharWidth(),
            y1 = ex->y / font.CharHeight(),
            x2 = (ex->x + ex->width - (2*config.BorderSize.LeftRight)) / font.CharWidth(),
            y2 = (ex->y + ex->height - (2*config.BorderSize.TopBottom)) / font.CharHeight();
        D("   Char exposed: (%d,%d) - (%d %d)", x1, y1, x2, y2);
        for(int x=max(x1, 0); x<min(x2, matrix.Width()); ++x) {
            for(int y=max(y1, 0); y<min(y2, matrix.Height()); ++y) {
                auto const& cell = matrix.Cells(x, y);
                DrawChar(x, y, cell.c, cell.attr);
            }
        }
        xcb_flush(c);
        break;
    }
    case XCB_KEY_PRESS: {
            char chr[5] = { 0, 0, 0, 0, 0 };
            keyboard.ParseKeyPress(reinterpret_cast<xcb_key_press_event_t*>(e), chr);
        break;
    }
    case XCB_DESTROY_NOTIFY:
        D("Quit event detected.");
        active = false;
        break;
    default:
        keyboard.ParseGenericEvent(e);
        break;
    }
    free(e);
    return { { NOTHING } }; 
}
    

void 
XcbRenderer::DrawChar(int x, int y, const char chr[4], Attributes attr) const
{
    char ch[4];
    memcpy(ch, chr, 4);

    // TODO - memoize cursor?

    // is it blinking? then hide the characters
    if(matrix.Blinking() && attr.blink) {
        ch[0] = ' '; ch[1] = 0;
    }

    // draw cursor background
    assert(config.CursorType == BACKGROUND);
    const bool is_cursor = (matrix.cursor.intensity != Cursor::INVISIBLE) && 
        static_cast<P>(matrix.cursor) == P{x,y};
    if(is_cursor) {
        attr.bg_color = matrix.cursor.color();
    }
    
    // find position
    int px = config.BorderSize.LeftRight + (x * font.CharWidth());
    int py = config.BorderSize.TopBottom + (y * font.CharHeight());

    // draw
    xcb_copy_area(c, GetCharPixmap(ch, attr), window, gc, 
            0, 0, px, py, font.CharWidth(), font.CharHeight());
}


void 
XcbRenderer::RedrawBorder() const
{
    xcb_rectangle_t rs[] = { 
        { 0, 0, config.BorderSize.LeftRight, win_h+config.BorderSize.TopBottom }, 
        { win_w+config.BorderSize.LeftRight, 0, config.BorderSize.LeftRight, (win_h+2*config.BorderSize.TopBottom) }, 
        { 0, 0, (win_w+config.BorderSize.LeftRight), config.BorderSize.TopBottom },
        { 0, win_h+config.BorderSize.TopBottom, (win_w+config.BorderSize.LeftRight), config.BorderSize.TopBottom },
    };

    SetGCForeground(config.BorderColor);
    xcb_poly_fill_rectangle(c, window, gc, 4, rs);
}


uint32_t 
XcbRenderer::GetCharPixmap(const char ch[4], Attributes const& attr) const
{
    Cell cell{ { ch[0], ch[1], ch[2], ch[3] }, attr };
    auto ci = ch_pixmaps.find(cell);
    if(ci == ch_pixmaps.end()) {

        // create pixmap
        uint32_t px = xcb_generate_id(c);
        CharImage img = font.LoadChar(ch, attr);
        xcb_create_pixmap(c, depth, px, window, img.w, img.h);

        // draw background
        SetGCForeground(attr.bg_color);
        xcb_rectangle_t r[] = { { 0, 0, img.w, img.h } };
        xcb_poly_fill_rectangle(c, px, gc, 1, r);

        // create list points for each color
        map<Color, vector<xcb_point_t>> pts = {};
        for(int i=0; i<(img.w * img.h); ++i) {
            Color color = img.data[i];
            if(color != attr.bg_color) {
                xcb_point_t pos { (i % img.w), (i / img.w) };
                auto cc = pts.find(color);
                if(cc == pts.end()) {
                    pts[color] = { move(pos) };
                } else {
                    cc->second.emplace_back(move(pos));
                }
            }
        }

        // draw foreground
        for(auto const& kv: pts) {
            SetGCForeground(kv.first);
            xcb_poly_point(c, XCB_COORD_MODE_ORIGIN, px, gc, kv.second.size(), &kv.second[0]);
        }
        
        // store in map
        ch_pixmaps[cell] = px;
        /* XXX: important! This might represent a leak in the X server. 
         * It's important to remember that, if the pixmaps are no longer
         * needed (for example, when the user changes the font) to call
         * xcb_free_pixmap for the pixmaps stored. */

        D("Created char for '%s' (0x%x...).", ch, ch[0]);
        return px;
    } else {
        return ci->second;
    }
}


uint32_t
XcbRenderer::GetColor(Color const& color) const
{
    // look for color in memo -- if not found, allocate it
    auto ci = colors.find(color);
    if(ci == colors.end()) {
        xcb_alloc_color_reply_t* rep = xcb_alloc_color_reply(c,
                xcb_alloc_color(c, colormap, color.r*0x100, color.g*0x100, color.b*0x100), 
                nullptr);
        colors[color] = unique_ptr<struct xcb_alloc_color_reply_t,
            function<void(xcb_alloc_color_reply_t*)>>(rep, [](xcb_alloc_color_reply_t* r) {
                D("Color #%02X%02X%02X deallocated.", (r->red/0x100), (r->green/0x100), (r->blue/0x100));
                free(r);
            });
        D("Color #%02X%02X%02X allocated.", color.r, color.g, color.b);
        return rep->pixel;
    } else {
        return ci->second->pixel;
    }
}


void 
XcbRenderer::SetGCForeground(Color const& color) const
{
    uint32_t p = GetColor(color);
    const uint32_t value[1] = { p };
    xcb_change_gc(c, gc, XCB_GC_FOREGROUND, value);
}


// vim: ts=4:sw=4:sts=4:expandtab
