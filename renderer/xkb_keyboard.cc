#include "xkb_keyboard.h"

#include <cassert>
#include <cstdlib>

#include <xcb/xcb.h>
#include <xkbcommon/xkbcommon.h>
#include <xkbcommon/xkbcommon-x11.h>
#include <xkbcommon/xkbcommon-compose.h>

#include "debug.h"
#include "renderer.h"
#include <xkb.h>  // renderer/system/xkb.h

XkbKeyboard::XkbKeyboard(struct xcb_connection_t *c)
{
    // Some links:
    //   http://xkbcommon.org/doc/current/md_doc_quick-guide.html
    //   https://github.com/xkbcommon/libxkbcommon/blob/master/test/interactive-x11.c
    //   Compose: https://github.com/xkbcommon/libxkbcommon/commit/5cefa5c5d09a89c902967c2ec5d4dcb3a6592781

    int ret = xkb_x11_setup_xkb_extension(c,
                                      XKB_X11_MIN_MAJOR_XKB_VERSION,
                                      XKB_X11_MIN_MINOR_XKB_VERSION,
                                      XKB_X11_SETUP_XKB_EXTENSION_NO_FLAGS,
                                      nullptr, nullptr, &first_xkb_event, nullptr);
    assert(ret);

    ctx = xkb_context_new(XKB_CONTEXT_NO_FLAGS);
    if(!ctx) {
        throw RendererInitException("Could not initialize xkb context.");
    }

    device_id = xkb_x11_get_core_keyboard_device_id(c);
    if(device_id == -1) {
        throw RendererInitException("Could not find xkb keyboard device.");
    }
    
    keymap = xkb_x11_keymap_new_from_device(ctx, c, device_id, XKB_KEYMAP_COMPILE_NO_FLAGS);
    if(!keymap) {
        throw RendererInitException("Could not initialize xkb keymap.");
    }
    
    state = xkb_x11_state_new_from_device(keymap, c, device_id);
    if(!state) {
        throw RendererInitException("Could not initialize xkb state.");
    }

    D("Xkb initialized.");

    const char *locale = setlocale(LC_CTYPE, nullptr);  // TODO
    D("Using locale: %s", locale);
    
    compose_table = xkb_compose_table_new_from_locale(ctx, locale, XKB_COMPOSE_COMPILE_NO_FLAGS);
    if(!compose_table) {
        throw RendererInitException("Could not initialize xkb compose table.");
    }

    compose_state = xkb_compose_state_new(compose_table, XKB_COMPOSE_STATE_NO_FLAGS);
    if(!compose_state) {
        throw RendererInitException("Could not initialize xkb compose state.");
    }

    this->SetupEventsFilter(c);
}


XkbKeyboard::~XkbKeyboard()
{
    xkb_compose_table_unref(compose_table);
    xkb_compose_state_unref(compose_state);
    xkb_state_unref(state);
    xkb_keymap_unref(keymap);
    xkb_context_unref(ctx);
}


void 
XkbKeyboard::SetupEventsFilter(struct xcb_connection_t *c)
{
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
        .affectCtrls = 0,
        .ctrlDetails = 0,
        .affectIndicatorState = 0,
        .indicatorStateDetails = 0,
        .affectIndicatorMap = 0,
        .indicatorMapDetails = 0,
        .affectNames = 0,
        .namesDetails = 0,
        .affectCompat = 0,
        .compatDetails = 0,
        .affectBell = 0,
        .bellDetails = 0,
        .affectMsgDetails = 0,
        .msgDetails = 0,
        .affectAccessX = 0,
        .accessXDetails = 0,
        .affectExtDev = 0,
        .extdevDetails = 0,
    };

    /* xcb_void_cookie_t cookie = */
        xcb_xkb_select_events_aux_checked(c,
                                          device_id,
                                          required_events,    /* affectWhich */
                                          0,                  /* clear */
                                          0,                  /* selectAll */
                                          required_map_parts, /* affectMap */
                                          required_map_parts, /* map */
                                          &details);          /* details */
}


void 
XkbKeyboard::ParseKeyPress(xcb_key_press_event_t* ev, char chr[5]) const {
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
    (void) compose_status;

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

    if(status == XKB_COMPOSE_COMPOSED) {
        keysym = xkb_compose_state_get_one_sym(compose_state);
        int n = xkb_compose_state_get_utf8(compose_state, chr, 5); (void) n;
        D("   compose key pressed:     '%s' (%d utf-8 bytes)", chr, n);
    } else {
        int n = xkb_state_key_get_utf8(state, ev->detail, chr, 5); (void) n;

        if(chr[0] >= 32) {
            D("   regular key pressed:     '%s' (%d utf-8 bytes)", chr, n);
        } else {
            D("   regular key pressed:     0x%02X", chr[0]);
        }

        if(status != XKB_COMPOSE_NOTHING) {
            chr[0] = 0;  // don't return anything when COMPOSING or CANCELLED
        }
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
    } *event = reinterpret_cast<union xkb_event *>(ev);

    if (event->any.deviceID != device_id) {
        return;
    }
    
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
