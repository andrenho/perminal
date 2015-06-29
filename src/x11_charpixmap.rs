extern crate x11;
use self::x11::xlib;

extern crate libc;
use self::libc::c_uint;

use font::Font;
use matrix::Attributes;

pub struct X11CharPixmap {
    pub pixmap: xlib::Pixmap,
    display: *mut xlib::Display,
}

impl X11CharPixmap {
    pub fn new(display: *mut xlib::Display, window: c_uint, depth: i32, font: &Font, c: char, attr: &Attributes) -> Self {
        let px = unsafe { xlib::XCreatePixmap(display, window, font.char_width(), font.char_height(), depth as u32) };
        // xlib::XDrawPoint(self.display, px, self.gc, 5, 5);
        // TODO - draw character
        X11CharPixmap { display: display, pixmap: px }
    }
}

impl Drop for X11CharPixmap {
    fn drop(&mut self) {
        unsafe { xlib::XFreePixmap(self.display, self.pixmap); }
    }
}


// vim: ts=4:sw=4:sts=4:expandtab
