#include "xcbrenderer.h"

#include <xcb/xcb.h>
#include <xkbcommon/xkbcommon.h>
#include <xkbcommon/xkbcommon-x11.h>
#include <xkbcommon/xkbcommon-compose.h>

#include <cassert>

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

    // TODO - initialize keyboard configuration
    // http://xkbcommon.org/doc/current/md_doc_quick-guide.html
    // https://github.com/xkbcommon/libxkbcommon/blob/master/test/interactive-x11.c
    // Compose: https://github.com/xkbcommon/libxkbcommon/commit/5cefa5c5d09a89c902967c2ec5d4dcb3a6592781

    uint8_t first_xkb_event;
    int ret = xkb_x11_setup_xkb_extension(c,
                                      XKB_X11_MIN_MAJOR_XKB_VERSION,
                                      XKB_X11_MIN_MINOR_XKB_VERSION,
                                      XKB_X11_SETUP_XKB_EXTENSION_NO_FLAGS,
                                      NULL, NULL, &first_xkb_event, NULL);
    assert(ret);

    struct xkb_context* ctx;
    ctx = xkb_context_new(XKB_CONTEXT_NO_FLAGS);
    assert(ctx);

    int32_t device_id = xkb_x11_get_core_keyboard_device_id(c);
    assert(device_id != -1);
    struct xkb_keymap* keymap = xkb_x11_keymap_new_from_device(ctx, c, device_id, XKB_KEYMAP_COMPILE_NO_FLAGS);
    assert(keymap);

    state = xkb_x11_state_new_from_device(keymap, c, device_id);
    assert(state);

    const char *locale = /*"pt_BR"; */ setlocale(LC_CTYPE, NULL);
    struct xkb_compose_table *compose_table = xkb_compose_table_new_from_locale(ctx, locale, XKB_COMPOSE_COMPILE_NO_FLAGS);
    assert(compose_table);
    compose_state = xkb_compose_state_new(compose_table, XKB_COMPOSE_STATE_NO_FLAGS);
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
        DrawChar(0, 0, 'A', { { 255, 255, 255 }, { 0, 0, 0 } });
        DrawChar(1, 0, 'B', { { 255, 255, 255 }, { 0, 0, 0 } });
        DrawChar(2, 0, 'C', { { 255, 255, 255 }, { 0, 0, 0 } });
        xcb_flush(c);
        break;
    case XCB_KEY_PRESS: {
            xcb_key_press_event_t* ev = reinterpret_cast<xcb_key_press_event_t*>(e);
            D("%d %d %d", ev->detail, ev->sequence, ev->state);
            xkb_keysym_t keysym = xkb_state_key_get_one_sym(state, ev->detail);
            D("%d", keysym);
            char keysym_name[64];
            xkb_keysym_get_name(keysym, keysym_name, sizeof(keysym_name));
            D("%s", keysym_name);

            char *buffer;
            int size = xkb_state_key_get_utf8(state, ev->detail, NULL, 0) + 1;
            buffer = (char*)calloc(size, 1);
            xkb_state_key_get_utf8(state, ev->detail, buffer, size);
            printf("%s\n", buffer);
            free(buffer);

            enum xkb_compose_status status = XKB_COMPOSE_NOTHING;
            status = xkb_compose_state_get_status(compose_state);
            D("%d", status);
        }
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

    // xcb_flush(c);
}


void 
XcbRenderer::DrawChar(int x, int y, char32_t ch, Attributes const& attr) const
{
    x = config.BorderSize.LeftRight + (x * font.CharWidth());
    y = config.BorderSize.TopBottom + (y * font.CharHeight());

    xcb_copy_area(c, GetCharPixmap(ch, attr), window, gc, 
            0, 0, x, y, font.CharWidth(), font.CharHeight());
}


void 
XcbRenderer::RedrawBorder() const
{
    xcb_rectangle_t rs[] = { 
        { 0, 0, config.BorderSize.LeftRight, win_h }, 
        { win_w-config.BorderSize.LeftRight, 0, config.BorderSize.LeftRight, win_h }, 
        { 0, 0, win_w, config.BorderSize.TopBottom },
        { 0, win_h-config.BorderSize.TopBottom, win_w, config.BorderSize.TopBottom },
    };

    SetGCForeground(config.BorderColor);
    xcb_poly_fill_rectangle(c, window, gc, 4, rs);
}


uint32_t 
XcbRenderer::GetCharPixmap(char32_t ch, Attributes const& attr) const
{
    Cell cell{ ch, attr };
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

        D("Created char for '%c' (0x%x).", ch, ch);
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
                D("Color #%02X%02X%02X deallocated.", (r->red/100), (r->green/100), (r->blue/100));
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
