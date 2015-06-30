use std::ffi::CString;
use std::ffi::CStr;
use std::str;
use std::ptr::{null,null_mut};
use std::mem::zeroed;
use std::mem::transmute;
use std::cell::Cell;
use std::cell::RefCell;
use std::collections::HashMap;

extern crate libc;
use self::libc::c_char;
use self::libc::c_uint;

extern crate x11;
use self::x11::xlib;

use renderer::Renderer;
use userevent::Key::*;
use userevent::UserEvent;
use userevent::UserEvent::*;
use matrix::*;
use chars::*;
use font::Font;
use x11_charpixmap::X11CharPixmap;
use x11_color::X11ColorAllocator;

pub struct X11Renderer<F:Font> {
    // associated values
    font:       F,
    color_a:    X11ColorAllocator,
    // X11 status
    display:    *mut xlib::Display,
    window:     xlib::Window,
    gc:         *mut xlib::_XGC,
    depth:      i32,
    im:         xlib::XIM,
    ic:         xlib::XIC,
    // mutable data
    char_pxmap: RefCell<HashMap<(char, Attributes), X11CharPixmap>>,
    active:     Cell<bool>,
}

impl<F:Font> X11Renderer<F> {
    pub fn new(font: F) -> Self {
        let display;
        let window;
        let screen_num;
        let gc;
        let depth;
        let im;
        let ic;
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
            xlib::XSelectInput(display, window, 
                xlib::StructureNotifyMask|xlib::SubstructureNotifyMask|xlib::KeyPressMask);

            // show window
            xlib::XMapWindow(display, window);

            // get default GC
            gc = xlib::XDefaultGC(display, xlib::XDefaultScreen(display));

            // find depth
            depth = xlib::XDefaultDepth(display, screen_num);
            
            // initialize i18n
            im = xlib::XOpenIM(display, null_mut(), null_mut(), null_mut());
            println!("{:?}", im);
            ic = xlib::XCreateIC(im);
            println!("{:?}", ic);
        }

        // create structure
        X11Renderer {
            font:       font,
            color_a:    X11ColorAllocator::new(display),
            display:    display,
            window:     window,
            gc:         gc,
            depth:      depth,
            im:         im,
            ic:         ic,
            active:     Cell::new(true),
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
            if xlib::XPending(self.display) > 0 {
                xlib::XNextEvent(self.display, &mut event);
                match event.get_type() {
                    xlib::DestroyNotify => {
                        self.active.set(false);
                        vec![]
                    },
                    xlib::KeyPress => {
                        let mut k_ev: &mut xlib::XKeyEvent = transmute(&mut event);
                        self.key_event(k_ev)
                    },
                    _ => vec![],
                }
            } else {
                vec![]
            }
        }
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
            X11CharPixmap::new(self.display, self.window, self.depth, &self.color_a, &self.font, c.c, &c.attr)
        });
        unsafe {
            xlib::XCopyArea(self.display, px.pixmap, self.window, self.gc,
                0, 0, w, h,
                (x as i32)*(w as i32), (y as i32)*(h as i32));
        }
    }


    unsafe fn key_event(&self, k_ev: &mut xlib::XKeyEvent) -> Vec<UserEvent> {
        /* TODO - support dead keys. This is pretty complex but can be
           done creating a input context and using XmbLookupString.
           See <http://www.sbin.org/doc/Xlib/chapt_11.html> */
        let mut key = [0 as c_char, 4]; // CString::new("    ").unwrap();
        let mut keysym: xlib::KeySym = zeroed();
        let mut status: xlib::Status = zeroed();
        let c = xlib::XmbLookupString(self.ic, k_ev, key.as_mut_ptr(), 4, &mut keysym, &mut status);
        println!("{} {}", c, key[0]);

        match xlib::XLookupKeysym(k_ev, 0) {
            c @ 1...255 => vec![KeyPress { key: Char(c as u8 as char), control: false, shift: false, alt: false }],
            c @ _ => {
                let k = match str::from_utf8(CStr::from_ptr(xlib::XKeysymToString(c)).to_bytes()).unwrap() {
                    "Return" => Some(Char(13 as char)),
                    "F12"    => Some(F12),
                    _        => None,
                };
                match k {
                    Some(k) => vec![KeyPress { key: k, control: false, shift: false, alt: false }],
                    None    => vec![],
                }
            },
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
