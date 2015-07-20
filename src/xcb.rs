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
#[repr(C)]
struct xcb_void_cookie_t {
    sequence: c_uint,
}

#[link(name = "xcb")]
extern {
    fn xcb_connect(display: *const c_char, screen: *const c_int) -> *mut c_void;
    fn xcb_disconnect(conn: *mut c_void) -> c_void;

    fn xcb_generate_id(conn: *mut c_void) -> u32;

    fn xcb_create_window(conn: *mut c_void, depth: u8, wid: xcb_window_t, parent: xcb_window_t,
                         x: i16, y: i16, w: u16, h: u16, border: u16, _class: u16, visual: xcb_visualid_t,
                         value_mask: u32, value_list: *const u32) -> xcb_void_cookie_t;
}

//
// CONNECTION
//
pub struct Connection {
    conn: xcb_connection_t,
    default_screen: c_int,
}

pub struct Window {
    id: xcb_window_t,
}

impl Connection {

    pub fn connect() -> Result<Self, &'static str> {
        let mut default_screen: c_int = 0;
        let conn = unsafe { xcb_connect(ptr::null(), &default_screen) };
        if !conn.is_null() {
            Ok(Connection { conn: conn, default_screen: default_screen })
        } else {
            Err("Unable to connect to display")
        }
    }

    fn create_simple_window(&self, w: u16, h: u16, border: u16, values: Vec<(u32, u32)>) -> Window {
        let w = Window { id: unsafe { xcb_generate_id(self.conn) } };
        xcb_create_window(self.conn, 0 /* copy from parent */, w.id, 
    //fn xcb_create_window(conn: *mut c_void, depth: u8, wid: xcb_window_t, parent: xcb_window_t,
    //                     x: i16, y: i16, w: u16, h: u16, border: u16, _class: u16, visual: xcb_visualid_t,
    //                     value_mask: u32, value_list: *const u32) -> xcb_void_cookie_t;
        w
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
        Connection::connect().unwrap();
    }

}

// vim: ts=4:sw=4:sts=4:expandtab
