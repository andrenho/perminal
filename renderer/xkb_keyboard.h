#ifndef XKB_KEYBOARD_H
#define XKB_KEYBOARD_H

#include <cstdint>
#include <xcb/xcb.h>

class XkbKeyboard {
public:
    explicit XkbKeyboard(struct xcb_connection_t *c);
    ~XkbKeyboard();

    void ParseKeyPress(struct xcb_key_press_event_t* ev) const;
    void ParseGenericEvent(xcb_generic_event_t* ev) const;

private:
    struct xcb_connection_t *c;

    struct xkb_state* state;
    uint8_t first_xkb_event;
    struct xkb_compose_state *compose_state;

    XkbKeyboard(XkbKeyboard const&) = delete;
    XkbKeyboard(XkbKeyboard&&) = delete;
    XkbKeyboard& operator=(XkbKeyboard const&) = delete;
    XkbKeyboard& operator=(XkbKeyboard&&) = delete;
};

#endif

// vim: ts=4:sw=4:sts=4:expandtab
