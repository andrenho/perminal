use userevent::UserEvent;
use userevent::UserEvent::*;
use userevent::Key::*;
use matrix::*;
use renderer::Renderer;

extern crate ncurses;
use self::ncurses::*;

pub struct CursesRenderer;


impl CursesRenderer {
    pub fn new() -> CursesRenderer { 
        initscr();
        cbreak();
        noecho();
        keypad(stdscr, true);
        timeout(0);
        curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE);
        refresh();
        CursesRenderer
    }
}


impl Renderer for CursesRenderer {
    fn is_running(&self) -> bool { 
        true 
    }

    fn get_user_input(&self) -> Vec<UserEvent> { 
        match getch() {
            KEY_F12 => {
                vec![KeyPress { key: F12, control: false, shift: false, alt: false }]
            },
            10 => {
                vec![KeyPress { key: Char(13 as char), control: false, shift: false, alt: false }]
            },
            c @ 32...128 => {
                vec![KeyPress { key: Char(c as u8 as char), control: false, shift: false, alt: false }]
            },
            _ => vec![],
        }
    }

    fn update(&self, matrix: &mut Matrix) {
        for dirty in matrix.dirty().iter() {
            let x = dirty.x;
            let y = dirty.y;
            self.draw_char(matrix, x, y);
        }
        match matrix.cursor_on {
            true  => { curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE); () },
            false => { curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE); () },
        }
        wmove(stdscr, matrix.cursor.y as i32, matrix.cursor.x as i32);
        refresh();
    }


}

impl CursesRenderer {

    fn get_attribute(&self, attr: Attributes) -> attr_t {
        let mut a = A_NORMAL();
        if attr.standout { a |= A_STANDOUT(); }
        if attr.underline { a |= A_UNDERLINE(); }
        a
    }

    fn draw_char(&self, matrix: &Matrix, x: u16, y: u16) {
        let c = matrix.cells[&P(x,y)];
        let ch = match c.c as u32 {
            27 => ACS_DIAMOND(),
            c @ 32...255 => c as u64,
            _ => ACS_STERLING(),
        };
        mvaddch(y as i32, x as i32, ch as u64);
        mvchgat(y as i32, x as i32, 1, self.get_attribute(c.attr), 0);
    }

}

/*
        // hack because mvaddch signature is different in x86 and i64
        #[cfg(target_pointer_width = "32")]
        fn ch(x: u16, y: u16, c: u32) { mvaddch(y as i32, x as i32, c); }
        #[cfg(target_pointer_width = "64")]
        fn ch(x: u16, y: u16, c: u64) { mvaddch(y as i32, x as i32, c); }

            ch(x, y, match matrix.cells[&P(x,y)].c as u64 {
                127 => ACS_STERLING(),
                c @ 32...255 => c as u64,
                27 => ACS_DIAMOND(),
                _ => ACS_STERLING(),
            });
*/

impl Drop for CursesRenderer {
    fn drop(&mut self) {
        endwin();
    }
}


// vim: ts=4:sw=4:sts=4:expandtab
