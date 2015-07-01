#include "xcbrenderer.h"

#include <xcb/xcb.h>

#include "config.h"
#include "debug.h"

XcbRenderer::XcbRenderer(Font const& font) 
    : font(font), c(xcb_connect(nullptr, nullptr)), window(xcb_generate_id(c))
{
    // check open connection
    if(!c) {
        throw "could not connect to a X server";
    }
    D("Connected to a X server.");

    // get screen information
    int screen_nbr;
    xcb_screen_t* screen;
    xcb_screen_iterator_t iter = xcb_setup_roots_iterator(xcb_get_setup(c));
    for(; iter.rem; --screen_nbr, xcb_screen_next(&iter)) {
        screen = iter.data;
        break;
    }
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
    };

    // create colormap & draw border
    colormap = screen->default_colormap;

    // create window
    xcb_create_window(c, 
            XCB_COPY_FROM_PARENT,           // depth
            window,                         // id
            screen->root,                   // parent
            0, 0,                           // position
            win_w, win_h,                   // dimenstions
            0,                              // border width
            XCB_WINDOW_CLASS_INPUT_OUTPUT,  // class
            screen->root_visual,            // visual
            mask, values);                  // masks
    xcb_map_window(c, window);
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


vector<UserEvent> 
XcbRenderer::GetEvents() const 
{ 
    xcb_generic_event_t* e = xcb_wait_for_event(c);
    switch(e->response_type & ~0x80) {
    case XCB_EXPOSE:
        D("Expose event detected.");
        RedrawBorder();
        xcb_flush(c);
        break;
    case XCB_DESTROY_NOTIFY:
        D("Quit event detected.");
        active = false;
        break;
    }
    free(e);
    return {}; 
}
    

void 
XcbRenderer::Update(Matrix const& matrix) const 
{ 
    (void) matrix;
}


void 
XcbRenderer::RedrawBorder() const
{
    uint32_t p = GetColor(config.BorderColor);
    const uint32_t value[1] = { p };
    xcb_change_gc(c, gc, XCB_GC_FOREGROUND, value);

    xcb_rectangle_t rs[] = { 
        { 0, 0, config.BorderSize.LeftRight, win_h }, 
        { win_w-config.BorderSize.LeftRight, 0, config.BorderSize.LeftRight, win_h }, 
        { 0, 0, win_w, config.BorderSize.TopBottom },
        { 0, win_h-config.BorderSize.TopBottom, win_w, config.BorderSize.TopBottom },
    };
    xcb_poly_fill_rectangle(c, window, gc, 4, rs);
}


uint32_t
XcbRenderer::GetColor(Color const& color) const
{
    // look for color in memo -- if not found, allocate it
    auto ci = colors.find(color);
    if(ci == colors.end()) {
        xcb_alloc_color_reply_t* rep = xcb_alloc_color_reply(c,
                xcb_alloc_color(c, colormap, color.r*100, color.g*100, color.b*100), 
                nullptr);
        colors[color] = unique_ptr<struct xcb_alloc_color_reply_t,
            function<void(xcb_alloc_color_reply_t*)>>(rep, [](xcb_alloc_color_reply_t* r) {
                D("Color #%02X%02X%02X deallocated.", (r->red/100), (r->green/100), (r->blue/100));
                free(r);
            });
        D("Color #%02X%02X%02X allocated.", color.r, color.g, color.b);
        return rep->pixel;
    } else {
        return ci->second->pixel;
    }
}


// vim: ts=4:sw=4:sts=4:expandtab
