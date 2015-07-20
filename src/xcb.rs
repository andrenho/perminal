#![allow(non_camel_case_types)]

extern crate libc;
use self::libc::*;
use std::ptr;

//
// XCB
//
type xcb_connection_t = *mut c_void;
type xcb_window_t = u32;
type xcb_visualid_t = u32;
type xcb_colormap_t = u32;

#[repr(C)]
struct xcb_void_cookie_t {
    sequence: c_uint,
}

#[repr(C)]
struct xcb_screen_t {
    root:                  xcb_window_t,
    default_colormap:      xcb_colormap_t,
    white_pixel:           u32,
    black_pixel:           u32,
    current_input_masks:   u32,
    width_in_pixels:       u16,
    height_in_pixels:      u16,
    width_in_millimeters:  u16,
    height_in_millimeters: u16,
    min_installed_maps:    u16,
    max_installed_maps:    u16,
    root_visual:           xcb_visualid_t,
    backing_stores:        u8,
    save_unders:           u8,
    root_depth:            u8,
    allowed_depths_len:    u8,
}

#[repr(C)]
struct xcb_screen_iterator_t {
    data:   *const xcb_screen_t,
    rem:    c_int,
    index:  c_int,
}

#[link(name = "xcb")]
extern {
    fn xcb_connect(display: *const c_char, screen: *const c_int) -> *mut c_void;
    fn xcb_disconnect(conn: xcb_connection_t) -> c_void;

    fn xcb_generate_id(conn: xcb_connection_t) -> u32;

    fn xcb_get_setup(conn: xcb_connection_t) -> *mut c_void;
    fn xcb_setup_roots_iterator(setup: *mut c_void) -> xcb_screen_iterator_t;
    fn xcb_screen_next(i: *mut xcb_screen_iterator_t) -> c_void;

    fn xcb_create_window(conn: *mut c_void, depth: u8, wid: xcb_window_t, parent: xcb_window_t,
                         x: i16, y: i16, w: u16, h: u16, border: u16, _class: u16, visual: xcb_visualid_t,
                         value_mask: u32, value_list: *const u32) -> xcb_void_cookie_t;
    fn xcb_map_window(conn: *mut c_void, wid: xcb_window_t) -> xcb_void_cookie_t;
}

// 
// EVENTS
//
const XCB_EVENT_MASK_NO_EVENT : u32 = 0;
const XCB_EVENT_MASK_KEY_PRESS : u32 = 1;
const XCB_EVENT_MASK_KEY_RELEASE : u32 = 2;
const XCB_EVENT_MASK_BUTTON_PRESS : u32 = 4;
const XCB_EVENT_MASK_BUTTON_RELEASE : u32 = 8;
const XCB_EVENT_MASK_ENTER_WINDOW : u32 = 16;
const XCB_EVENT_MASK_LEAVE_WINDOW : u32 = 32;
const XCB_EVENT_MASK_POINTER_MOTION : u32 = 64;
const XCB_EVENT_MASK_POINTER_MOTION_HINT : u32 = 128;
const XCB_EVENT_MASK_BUTTON_1_MOTION : u32 = 256;
const XCB_EVENT_MASK_BUTTON_2_MOTION : u32 = 512;
const XCB_EVENT_MASK_BUTTON_3_MOTION : u32 = 1024;
const XCB_EVENT_MASK_BUTTON_4_MOTION : u32 = 2048;
const XCB_EVENT_MASK_BUTTON_5_MOTION : u32 = 4096;
const XCB_EVENT_MASK_BUTTON_MOTION : u32 = 8192;
const XCB_EVENT_MASK_KEYMAP_STATE : u32 = 16384;
const XCB_EVENT_MASK_EXPOSURE : u32 = 32768;
const XCB_EVENT_MASK_VISIBILITY_CHANGE : u32 = 65536;
const XCB_EVENT_MASK_STRUCTURE_NOTIFY : u32 = 131072;
const XCB_EVENT_MASK_RESIZE_REDIRECT : u32 = 262144;
const XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY : u32 = 524288;
const XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT : u32 = 1048576;
const XCB_EVENT_MASK_FOCUS_CHANGE : u32 = 2097152;
const XCB_EVENT_MASK_PROPERTY_CHANGE : u32 = 4194304;
const XCB_EVENT_MASK_COLOR_MAP_CHANGE : u32 = 8388608;
const XCB_EVENT_MASK_OWNER_GRAB_BUTTON : u32 = 1677721;

//
// CONNECTION
//
pub struct Connection {
    conn: xcb_connection_t,
    default_screen: c_int,
    screen: *const xcb_screen_t,
}

pub struct Window {
    id: xcb_window_t,
}

impl Connection {

    pub fn connect() -> Result<Self, &'static str> {
        // open connection
        let mut default_screen: c_int = 0;
        let conn = unsafe { xcb_connect(ptr::null(), &default_screen) };
        if conn.is_null() {
            Err("Unable to connect to display")
        } else {
            match Connection::screen_of_display(&conn, default_screen) {
                Ok(s)  => Ok(Connection { conn: conn, default_screen: default_screen, screen: s }),
                Err(e) => Err(e)
            }
        }
    }

    pub fn create_simple_window(&self, w: u16, h: u16, window: &'static str, events: &[u32]) -> Window {
        const XCB_COPY_FROM_PARENT: u8 = 0;
        const XCB_WINDOW_CLASS_INPUT_OUTPUT: u16 = 0;
        const XCB_CW_BACK_PIXEL: u32 = 2;
        const XCB_CW_EVENT_MASK: u32 = 2048;

        let mask = XCB_CW_BACK_PIXEL | XCB_CW_EVENT_MASK;
        let values = [ unsafe { (*self.screen).white_pixel }, 
                       events.into_iter().fold(0, |acc,&i| acc | &i) 
                            | XCB_EVENT_MASK_STRUCTURE_NOTIFY 
                            | XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY ];
        
        let win = Window { id: unsafe { xcb_generate_id(self.conn) } };
        unsafe { 
            xcb_create_window(self.conn, 
                              XCB_COPY_FROM_PARENT, 
                              win.id, 
                              (*self.screen).root, 
                              0, 0, w, h, 0, 
                              XCB_WINDOW_CLASS_INPUT_OUTPUT, 
                              (*self.screen).root_visual,
                              mask, values.as_ptr());
        }

        // TODO add window title

        win
    }

    fn screen_of_display(conn: &xcb_connection_t, screen: c_int) -> Result<*const xcb_screen_t, &'static str> {
        let mut screen = screen;
        let mut iter = unsafe { xcb_setup_roots_iterator(xcb_get_setup(*conn)) };
        loop {
            if iter.rem == 0 {
                return Err("A screen was not found");
            }
            if screen == 0 {
                return Ok(iter.data);
            }
            screen -= 1;
            unsafe { xcb_screen_next(&mut iter) };
        }
    }

}

impl Drop for Connection {
    
    fn drop(&mut self) {
        unsafe { xcb_disconnect(self.conn); }
    }

}

//
// TESTS
//
#[cfg(test)]
mod tests {

    use super::Connection;

    // 
    // TEST COMMANDS
    //

    #[test] 
    fn open_connection() {
        let c = Connection::connect().unwrap();
        c.create_simple_window(640, 480, "test", &[]);
    }

}

// vim: ts=4:sw=4:sts=4:expandtab
