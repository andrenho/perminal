#![allow(non_camel_case_types)]

extern crate libc;
use self::libc::*;
use std::ptr;

use xcb::xcb_connection_t;

//
// XKB
//
type xkb_context = c_void;
type xkb_keymap = c_void;
type xkb_state = c_void;
type xkb_compose_table = c_void;

#[link(name = "xcb-xkb")]
#[link(name = "xkbcommon-x11")]
#[link(name = "xkbcommon")]
extern {
    fn setlocale(category: c_int, locale: *const c_char) -> *mut c_char;
    
    fn xkb_x11_setup_xkb_extension(c: xcb_connection_t, major_xkb_version: u16, minor_xkb_version: u16,
        flags: c_int, major_xkb_version_out: *mut u16, minor_xkb_version_out: *mut u16, 
        base_event_out: *mut u8, base_error_out: *mut u8) -> c_int;
    fn xkb_context_new(flags: c_int) -> *mut xkb_context;
    fn xkb_x11_get_core_keyboard_device_id(c: xcb_connection_t) -> i32;
    fn xkb_x11_keymap_new_from_device(ctx: *mut xkb_context, c: xcb_connection_t, device_id: i32, 
        flags: c_int) -> *mut xkb_keymap;
    fn xkb_x11_state_new_from_device(ctx: *mut xkb_context, c: xcb_connection_t, device_id: i32) -> *mut xkb_state;
    fn xkb_compose_table_new_from_locale(ctx: *mut xkb_context, locale: *const c_char, flags: c_int) -> *mut xkb_compose_table;
}

//
// KEYBOARD
//
pub struct Keyboard;

impl Keyboard {

    fn new(c: xcb_connection_t) -> Self {
        // Some links:
        //   http://xkbcommon.org/doc/current/md_doc_quick-guide.html
        //   https://github.com/xkbcommon/libxkbcommon/blob/master/test/interactive-x11.c
        //   Compose: https://github.com/xkbcommon/libxkbcommon/commit/5cefa5c5d09a89c902967c2ec5d4dcb3a6592781

        unsafe { 
            let mut first_xkb_event = 0_u8;
            let ret = xkb_x11_setup_xkb_extension(c, 1, 0, 0, ptr::null_mut(), ptr::null_mut(), 
                &mut first_xkb_event, ptr::null_mut());
            assert!(ret != 0);

            let ctx = xkb_context_new(0);
            assert!(ctx != ptr::null_mut());

            let device_id = xkb_x11_get_core_keyboard_device_id(c);
            assert!(device_id != -1);

            let keymap = xkb_x11_keymap_new_from_device(ctx, c, device_id, 0);
            assert!(keymap != ptr::null_mut());

            let state = xkb_x11_state_new_from_device(ctx, c, device_id);
            assert!(state != ptr::null_mut());

            const LC_CTYPE: c_int = 0;
            let locale = setlocale(LC_CTYPE, ptr::null());

            let compose_table = xkb_compose_table_new_from_locale(ctx, locale, 0);
            assert!(compose_table != ptr::null_mut());
/*

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
*/        
        }
        Keyboard
    }

}

//
// TESTS
//
#[cfg(test)]
mod tests {

    use super::Keyboard;
    use xcb::Connection;

    #[test] 
    fn keyboard() {
        let c = Connection::connect().unwrap();
        let k = Keyboard::new(c.conn);
    }

}

// vim: ts=4:sw=4:sts=4:expandtab
