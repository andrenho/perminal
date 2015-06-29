use std::ffi::CString;
use std::ptr::{null,null_mut};
use std::mem::zeroed;
use std::cell::Cell;
use std::cell::RefCell;
use std::collections::HashMap;

extern crate libc;
use self::libc::c_uint;

extern crate x11;
use self::x11::xlib;

use renderer::Renderer;
use userevent::UserEvent;
use userevent::UserEvent::*;
use matrix::*;
use chars::*;
use font::Font;
use x11_charpixmap::X11CharPixmap;

pub struct X11Renderer<F:Font> {
    active: Cell<bool>,
    font: F,
    display: *mut xlib::Display,
    #[allow(dead_code)] screen_num: i32,
    window: xlib::Window,
    gc: *mut xlib::_XGC,
    depth: i32,
    char_pxmap: RefCell<HashMap<(char, Attributes), X11CharPixmap>>,
}

impl<F:Font> X11Renderer<F> {
    pub fn new(font: F) -> Self {
        let display;
        let window;
        let screen_num;
        let gc;
        let depth;
        unsafe {
            // open display
            display = xlib::XOpenDisplay(null());
            if display == null_mut() {
                panic!("can't open display!");
            }

            // create window
            screen_num = xlib::XDefaultScreen(display);
            let mut attributes: xlib::XSetWindowAttributes = zeroed();
            attributes.background_pixel = xlib::XWhitePixel(display, screen_num);
            window = xlib::XCreateWindow(
                display,
                xlib::XRootWindow(display, screen_num),
                0, 0, 640, 480, 0, 0,
                xlib::InputOutput as c_uint, null_mut(),
                xlib::CWBackPixel, &mut attributes);

            // set window title
            let title_str = CString::new("perminal (dev)").unwrap();
            xlib::XStoreName(display, window, title_str.as_ptr() as *mut _);

            // select input
            xlib::XSelectInput(display, window, xlib::StructureNotifyMask|xlib::SubstructureNotifyMask);

            // show window
            xlib::XMapWindow(display, window);
            loop {
                let mut e: xlib::XEvent = zeroed();
                xlib::XNextEvent(display, &mut e);
                if e.get_type() == xlib::MapNotify {
                    break;
                }
            }

            // create GC
            gc = xlib::XCreateGC(display, window, 0, null_mut());

            // find depth
            depth = xlib::XDefaultDepth(display, screen_num);
        }

        // create structure
        X11Renderer {
            active:     Cell::new(true),
            font:       font,
            display:    display,
            window:     window,
            screen_num: screen_num,
            gc:         gc,
            depth:      depth,
            char_pxmap: RefCell::new(HashMap::new()),
        }
    }
}


impl<F:Font> Renderer for X11Renderer<F> {
    fn is_running(&self) -> bool {
    	self.active.get()
    }

    fn get_user_input(&self) -> Vec<UserEvent> {
        unsafe {
            let mut event: xlib::XEvent = zeroed();
            xlib::XNextEvent(self.display, &mut event);
            match event.get_type() {
                xlib::DestroyNotify => self.active.set(false),
                _ => (),
            }
        }
        vec![]
    }

    fn update(&self, matrix: &mut Matrix) {
        // draw chars
        for dirty in matrix.dirty().iter() {
            let x = dirty.x;
            let y = dirty.y;
            self.draw_char(matrix, x, y);
        }
        // TODO - set cursor intensity, position
        // TODO - play bell
        // TODO - reverse screen
        // TODO - refresh screen
    }
}


impl<F:Font> X11Renderer<F> {
    fn draw_char(&self, matrix: &Matrix, x: u16, y: u16) {
        let c = matrix.cells[&P(x,y)];
        let w = self.font.char_width();
        let h = self.font.char_height();
        let mut m = self.char_pxmap.borrow_mut();
        let px = m.entry((c.c, c.attr)).or_insert_with(|| {
            X11CharPixmap::new(self.display, self.window, self.depth, &self.font, c.c, &c.attr)
        });
        unsafe {
            xlib::XCopyArea(self.display, px.pixmap, self.window, self.gc,
                0, 0, w, h,
                (x as i32)*(w as i32), (y as i32)*(h as i32));
        }
    }
}


/*
impl Drop for X11Renderer {
    fn drop(&mut self) {
    }
}
*/


// vim: ts=4:sw=4:sts=4:expandtab
