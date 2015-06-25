use userevent::UserEvent;
use userevent::UserEvent::*;
use userevent::Key::*;
use matrix::Matrix;
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
        // hack because mvaddch signature is different in x86 and i64
        #[cfg(target_pointer_width = "32")]
        fn ch(x: u16, y: u16, c: u32) { mvaddch(y as i32, x as i32, c); }
        #[cfg(target_pointer_width = "64")]
        fn ch(x: u16, y: u16, c: u64) { mvaddch(y as i32, x as i32, c); }

        for dirty in matrix.dirty().iter() {
            let x = dirty.0;
            let y = dirty.1;
            ch(x, y, match matrix.cells[&(x, y)].c as u64 {
                127 => ACS_STERLING(),
                c @ 32...255 => c as u32,
                27 => ACS_DIAMOND(),
                _ => ACS_STERLING(),
            });
        }
        match matrix.cursor_on {
            true  => { curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE); () },
            false => { curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE); () },
        }
        wmove(stdscr, matrix.cursor.1 as i32, matrix.cursor.0 as i32);
        refresh();
    }
}


impl Drop for CursesRenderer {
    fn drop(&mut self) {
        endwin();
    }
}


// vim: ts=4:sw=4:sts=4:expandtab
