#![allow(non_upper_case_globals)]

use std::mem::zeroed;
use std::collections::HashMap;

extern crate libc;
use self::libc::c_int;
use self::libc::c_ulong;

extern crate x11;
use self::x11::xlib;

use font::Font;
use chars::Attributes;
use chars::Color;
use x11_color::X11ColorAllocator;

const GCForeground: c_ulong = (1 << 2);
const GCBackground: c_ulong = (1 << 3);
const CoordModeOrigin: c_int = 0;

pub struct X11CharPixmap {
    pub pixmap: xlib::Pixmap,
    display:    *mut xlib::Display,
}

impl X11CharPixmap {
    pub fn new(display: *mut xlib::Display, window: xlib::Window, depth: i32, 
               color_a: &X11ColorAllocator, font: &Font, c: char, attr: &Attributes) -> Self {
        let px = unsafe { xlib::XCreatePixmap(display, window, font.char_width(), font.char_height(), depth as u32) };
        let chimg = font.load_char(c, attr);
        unsafe {
            let gc = xlib::XDefaultGC(display, xlib::XDefaultScreen(display));
            let mut values: xlib::XGCValues = zeroed();

            // draw background
            values.foreground = color_a.get(chimg.bg_color);
            xlib::XChangeGC(display, gc, GCForeground, &mut values);
            xlib::XFillRectangle(display, px, gc, 0, 0, font.char_width(), font.char_height());

            // create point lists for draw the character
            let mut pts: HashMap<Color, Vec<(xlib::XPoint)>> = HashMap::new();
            for x in 0..font.char_width() {
                for y in 0..font.char_height() {
                    let i = (x + (y * font.char_width())) as usize;
                    let color = chimg.data[i];
                    if color != chimg.bg_color {
                        let p = xlib::XPoint { x:x as i16, y:y as i16 };
                        pts.entry(color).or_insert(Vec::new()).push(p);
                    }
                }
            }

            // draw foreground
            for (color, xpts) in pts.iter_mut() {
                values.foreground = color_a.get(*color);
                xlib::XChangeGC(display, gc, GCForeground, &mut values);
                xlib::XDrawPoints(display, px, gc, &mut *xpts.as_mut_ptr(), xpts.len() as i32, CoordModeOrigin);
            }
        }
        X11CharPixmap { display: display, pixmap: px }
    }
}

impl Drop for X11CharPixmap {
    fn drop(&mut self) {
        unsafe { xlib::XFreePixmap(self.display, self.pixmap); }
    }
}


// vim: ts=4:sw=4:sts=4:expandtab
