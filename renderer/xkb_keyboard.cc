#include "xkb_keyboard.h"

#include <cassert>
#include <cstdlib>

#include <xcb/xcb.h>
#include <xkbcommon/xkbcommon.h>
#include <xkbcommon/xkbcommon-x11.h>
#include <xkbcommon/xkbcommon-compose.h>

#include "debug.h"
#include "xkb.h"

XkbKeyboard::XkbKeyboard(struct xcb_connection_t *c)
{
    // TODO - initialize keyboard configuration
    // http://xkbcommon.org/doc/current/md_doc_quick-guide.html
    // https://github.com/xkbcommon/libxkbcommon/blob/master/test/interactive-x11.c
    // Compose: https://github.com/xkbcommon/libxkbcommon/commit/5cefa5c5d09a89c902967c2ec5d4dcb3a6592781

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

    enum {
        required_events =
            (XCB_XKB_EVENT_TYPE_NEW_KEYBOARD_NOTIFY |
             XCB_XKB_EVENT_TYPE_MAP_NOTIFY |
             XCB_XKB_EVENT_TYPE_STATE_NOTIFY),

        required_nkn_details =
            (XCB_XKB_NKN_DETAIL_KEYCODES),

        required_map_parts =
            (XCB_XKB_MAP_PART_KEY_TYPES |
             XCB_XKB_MAP_PART_KEY_SYMS |
             XCB_XKB_MAP_PART_MODIFIER_MAP |
             XCB_XKB_MAP_PART_EXPLICIT_COMPONENTS |
             XCB_XKB_MAP_PART_KEY_ACTIONS |
             XCB_XKB_MAP_PART_VIRTUAL_MODS |
             XCB_XKB_MAP_PART_VIRTUAL_MOD_MAP),

        required_state_details =
            (XCB_XKB_STATE_PART_MODIFIER_BASE |
             XCB_XKB_STATE_PART_MODIFIER_LATCH |
             XCB_XKB_STATE_PART_MODIFIER_LOCK |
             XCB_XKB_STATE_PART_GROUP_BASE |
             XCB_XKB_STATE_PART_GROUP_LATCH |
             XCB_XKB_STATE_PART_GROUP_LOCK),
    };

    static const xcb_xkb_select_events_details_t details = {
        .affectNewKeyboard = required_nkn_details,
        .newKeyboardDetails = required_nkn_details,
        .affectState = required_state_details,
        .stateDetails = required_state_details,
    };

    xcb_void_cookie_t cookie =
        xcb_xkb_select_events_aux_checked(c,
                                          device_id,
                                          required_events,    /* affectWhich */
                                          0,                  /* clear */
                                          0,                  /* selectAll */
                                          required_map_parts, /* affectMap */
                                          required_map_parts, /* map */
                                          &details);          /* details */
    const char *locale = setlocale(LC_CTYPE, NULL);
    D("Keyboard locale: %s", locale);
    struct xkb_compose_table *compose_table = xkb_compose_table_new_from_locale(ctx, locale, XKB_COMPOSE_COMPILE_NO_FLAGS);
    assert(compose_table);
    compose_state = xkb_compose_state_new(compose_table, XKB_COMPOSE_STATE_NO_FLAGS);
}


XkbKeyboard::~XkbKeyboard()
{
}


void 
XkbKeyboard::ParseKeyPress(xcb_key_press_event_t* ev) const {
    // debugging info
    auto compose_status = [&](){
        switch(xkb_compose_state_get_status(compose_state)) {
            case XKB_COMPOSE_NOTHING:   return "NOTHING";
            case XKB_COMPOSE_COMPOSING: return "COMPOSING";
            case XKB_COMPOSE_COMPOSED:  return "COMPOSED";
            case XKB_COMPOSE_CANCELLED: return "CANCELLED";
            default:                    return "???";
        }
    };

    D("Key pressed:");
    D("   previous compose status: %s", compose_status());

    // get keysym
    xkb_keysym_t keysym = xkb_state_key_get_one_sym(state, ev->detail);

#ifdef DEBUG
    char keysym_name[64];
    xkb_keysym_get_name(keysym, keysym_name, sizeof(keysym_name));
    D("   key pressed (keysym):    '%s'", keysym_name);
#endif

    xkb_compose_state_feed(compose_state, keysym);
    auto status = xkb_compose_state_get_status(compose_state);

    char buffer[5];
    if(status == XKB_COMPOSE_COMPOSED) {
        keysym = xkb_compose_state_get_one_sym(compose_state);
        int n = xkb_compose_state_get_utf8(compose_state, buffer, 5);
        D("   compose key pressed:     '%s' (%d utf-8 bytes)", buffer, n);

        // TODO - return key
    } else {
        int n = xkb_state_key_get_utf8(state, ev->detail, buffer, 5);

        if(buffer[0] >= 32) {
            D("   regular key pressed:     '%s' (%d utf-8 bytes)", buffer, n);
        } else {
            D("   regular key pressed:     0x%02X", buffer[0]);
        }
        // TODO - return key
    }

    D("   new compose status:      %s", compose_status());
    D("----------------");
}


void 
XkbKeyboard::ParseGenericEvent(xcb_generic_event_t* ev) const
{
    union xkb_event {
        struct {
            uint8_t response_type;
            uint8_t xkbType;
            uint16_t sequence;
            xcb_timestamp_t time;
            uint8_t deviceID;
        } any;
        xcb_xkb_new_keyboard_notify_event_t new_keyboard_notify;
        xcb_xkb_map_notify_event_t map_notify;
        xcb_xkb_state_notify_event_t state_notify;
    } *event = (union xkb_event *) ev;

    /*
    if (event->any.deviceID != device_id)
        return;
    */
    
    switch(event->any.xkbType) {
    case XCB_XKB_STATE_NOTIFY:
        xkb_state_update_mask(state,
                              event->state_notify.baseMods,
                              event->state_notify.latchedMods,
                              event->state_notify.lockedMods,
                              event->state_notify.baseGroup,
                              event->state_notify.latchedGroup,
                              event->state_notify.lockedGroup);
    }
    
    /* TODO - notify keyboard switch, see
     * https://github.com/xkbcommon/libxkbcommon/blob/master/test/interactive-x11.c */
}


// vim: ts=4:sw=4:sts=4:expandtab
