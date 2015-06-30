extern crate x11;
use self::x11::xlib;
use std::mem::zeroed;

use font::Font;
use chars::Attributes;

pub struct X11CharPixmap {
    pub pixmap: xlib::Pixmap,
    display: *mut xlib::Display,
}

impl X11CharPixmap {
    pub fn new(display: *mut xlib::Display, window: xlib::Window, depth: i32, font: &Font, c: char, attr: &Attributes) -> Self {
        let px = unsafe { xlib::XCreatePixmap(display, window, font.char_width(), font.char_height(), depth as u32) };
        unsafe {
            let cmap = xlib::XDefaultColormap(display, xlib::XDefaultScreen(display));
            let mut bg_cell: xlib::XColor = zeroed();
            bg_cell.flags= xlib::DoRed | xlib::DoGreen | xlib::DoBlue; 
            bg_cell.red = (attr.bg_color.r as u16) * 0x100;
            bg_cell.green = (attr.bg_color.g as u16) * 0x100;
            bg_cell.blue = (attr.bg_color.b as u16) * 0x100;
            let bgc = xlib::XAllocColor(display, cmap, &mut bg_cell);
            let mut values: xlib::XGCValues = zeroed();
            values.foreground = bg_cell.pixel;
            values.background = bg_cell.pixel;
            let gc = xlib::XCreateGC(display, window, 0b1100 /* GCForeground */, &mut values);
            xlib::XFillRectangle(display, px, gc, 0, 0, font.char_width(), font.char_height());
            
            // xlib::XDrawPoint(self.display, px, self.gc, 5, 5);
            // TODO - draw character
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
