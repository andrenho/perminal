#ifndef XKB_KEYBOARD_H
#define XKB_KEYBOARD_H

#include <cstdint>
#include <xcb/xcb.h>

#include "userevent.h"

class XkbKeyboard {
public:
    explicit XkbKeyboard(struct xcb_connection_t *c);
    ~XkbKeyboard();

    SpecialKey ParseKeyPress(struct xcb_key_press_event_t* ev, char chr[5]) const;
    void ParseGenericEvent(xcb_generic_event_t* ev) const;

private:
    void SetupEventsFilter(struct xcb_connection_t *c);

    struct xkb_context* ctx = nullptr;
    struct xkb_state* state = nullptr;
    struct xkb_keymap* keymap = nullptr;
    uint8_t first_xkb_event = 0;
    int32_t device_id = 0;
    struct xkb_compose_state *compose_state = nullptr;
    struct xkb_compose_table *compose_table = nullptr;

    XkbKeyboard(XkbKeyboard const&) = delete;
    XkbKeyboard(XkbKeyboard&&) = delete;
    XkbKeyboard& operator=(XkbKeyboard const&) = delete;
    XkbKeyboard& operator=(XkbKeyboard&&) = delete;
};

#endif

// vim: ts=4:sw=4:sts=4:expandtab
