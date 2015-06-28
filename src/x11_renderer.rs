use std::cell::Cell;

use renderer::Renderer;
use userevent::UserEvent;
use userevent::UserEvent::*;
use matrix::*;

extern crate libc;
extern crate x11;

use std::ffi::CString;
use std::ptr::{null,null_mut};
use std::mem::zeroed;
use self::libc::c_uint;
use self::x11::xlib;

pub struct X11Renderer {
    active: Cell<bool>,
    display: *mut xlib::Display,
    window: u64,
}

impl X11Renderer {
    pub fn new() -> Self {
        let display;
        let window;
        unsafe {
            // open display
            display = xlib::XOpenDisplay(null());
            if display == null_mut() {
                panic!("can't open display!");
            }

            // create window
            let screen_num = xlib::XDefaultScreen(display);
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
        }

        // create structure
        X11Renderer {
            active: Cell::new(true),
            display: display,
            window: window,
        }
    }
}


impl Renderer for X11Renderer {
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
    }
}


/*
impl Drop for X11Renderer {
    fn drop(&mut self) {
    }
}
*/


// vim: ts=4:sw=4:sts=4:expandtab
