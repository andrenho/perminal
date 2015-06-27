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
        if matrix.play_bell {
            bell();
            matrix.play_bell = false;
        }
        wmove(stdscr, matrix.cursor.y as i32, matrix.cursor.x as i32);
        refresh();
    }


}

impl CursesRenderer {

    fn get_attribute(&self, attr: Attributes) -> attr_t {
        let mut a = A_NORMAL();
        if attr.standout  { a |= A_STANDOUT();   }
        if attr.underline { a |= A_UNDERLINE();  }
        if attr.reverse   { a |= A_REVERSE();    }
        if attr.blink     { a |= A_BLINK();      }
        if attr.bold      { a |= A_BOLD();       }
        if attr.dim       { a |= A_DIM();        }
        if attr.invisible { a |= A_INVIS();      }
        if attr.protected { a |= A_PROTECT();    }
        if attr.acs       { a |= A_ALTCHARSET(); }
        a
    }

    fn draw_char(&self, matrix: &Matrix, x: u16, y: u16) {
        let c = matrix.cells[&P(x,y)];
        let ch = match c.c as u32 {
            27 => ACS_DIAMOND() as u64,
            c @ 32...255 => c as u64,
            _ => ACS_STERLING() as u64,
        };
        mvaddch(y as i32, x as i32, ch as u64);
        mvchgat(y as i32, x as i32, 1, self.get_attribute(c.attr), 0);
    }

}


impl Drop for CursesRenderer {
    fn drop(&mut self) {
        endwin();
    }
}


// vim: ts=4:sw=4:sts=4:expandtab
