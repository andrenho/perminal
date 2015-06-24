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
        timeout(-1);
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
                vec![KeyPress { key: Char(13 as char), control: false, shift: false, alt: false },
                     KeyPress { key: Char(10 as char), control: false, shift: false, alt: false }]
            },
            c @ 32...128 => {
                vec![KeyPress { key: Char(c as u8 as char), control: false, shift: false, alt: false }]
            },
            _ => vec![],
        }
    }

    fn update(&self, matrix: &mut Matrix) {
        for dirty in matrix.dirty().iter() {
            let x = dirty.0;
            let y = dirty.1;
            mvaddch(y as i32, x as i32, matrix.cells[&(x, y)].c as u64);
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
