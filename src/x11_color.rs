use std::mem::zeroed;
use std::cell::RefCell;
use std::collections::HashMap;

extern crate libc;
use self::libc::c_ulong;

extern crate x11;
use self::x11::xlib;

use chars::Color;

pub struct X11ColorAllocator {
    display: *mut xlib::Display,
    cmap:    xlib::Colormap,
    index:   RefCell<HashMap<Color, c_ulong>>,
}

impl X11ColorAllocator {

    fn new(display: *mut xlib::Display) -> Self {
        unsafe {
            X11ColorAllocator {
                cmap:    xlib::XDefaultColormap(display, xlib::XDefaultScreen(display)),
                display: display,
                index:   RefCell::new(HashMap::new()),
            }
        }
    }

    fn get(&self, color: Color) -> c_ulong {
        let mut m = self.index.borrow_mut();
        *m.entry(color).or_insert_with(|| {
            unsafe {
                let mut cell: xlib::XColor = zeroed();
                cell.flags = xlib::DoRed | xlib::DoGreen | xlib::DoBlue; 
                cell.red   = (color.r as u16) * 0x100;
                cell.green = (color.g as u16) * 0x100;
                cell.blue  = (color.b as u16) * 0x100;
                xlib::XAllocColor(self.display, self.cmap, &mut cell);
                cell.pixel
            }
        })
    }

}
        

// vim: ts=4:sw=4:sts=4:expandtab
