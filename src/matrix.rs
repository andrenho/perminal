use command::Command;
use command::Command::*;
use std::cmp::{min,max};
use std::collections::HashMap;

//
// Position
//
#[derive(Clone,Copy,PartialEq,Eq,Hash)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}
#[allow(non_snake_case)]
pub fn P(x: u16, y: u16) -> Position { Position { x:x, y:y } }


//
// Attributes
//
#[derive(Clone,Copy)]
pub struct Attributes {
    pub standout: bool,
    pub underline: bool,
    pub reverse: bool,
    pub blink: bool,
    pub bold: bool,
    pub dim: bool,
    pub invis: bool,
    pub acs: bool,
}
impl Default for Attributes {
    fn default() -> Attributes {
        Attributes {
            standout: false,
            underline: false,
            reverse: false,
            blink: false,
            bold: false,
            dim: false,
            invis: false,
            acs: false,
        }
    }
}

//
// CharCell
//
#[derive(Clone,Copy)]
pub struct CharCell {
    pub c: char,
    pub attr: Attributes,
}
impl Default for CharCell {
    fn default() -> CharCell { CharCell { c: ' ', attr: Default::default() } }
}

//
// Matrix
//
pub struct Matrix {
    pub w: u16,
    pub h: u16,
    pub cells: HashMap<Position, CharCell>,
    pub cursor_on: bool,
    pub cursor: Position,
    saved_cursor: Position,
    dirty: Vec<Position>,
    current_attribute: Attributes,
    scroll_region: (u16,u16),
    insert_mode: bool,
}

impl Matrix {
    pub fn new(w: u16, h: u16) -> Matrix {
        let mut m = Matrix {
            w: w,
            h: h,
            cells: HashMap::new(),
            cursor: Position { x:0, y:0 },
            saved_cursor: Position { x:0, y:0 },
            cursor_on: true,
            dirty: vec![],
            current_attribute: Default::default(),
            scroll_region: (0, h-1),
            insert_mode: false,
        };
        for x in 0..w {
            for y in 0..h {
                m.cells.insert(Position{x:x, y:y}, Default::default());
            }
        }
        m
    }

    pub fn execute(&mut self, cmd: &Command) {
        match cmd {

            &IncompleteCommand => (),

            // Characters
            &PrintChar(c) => {
                let cursor = self.cursor;
                let attr = self.current_attribute;
                if self.insert_mode {
                    self.insert_chars(1);
                }
                self.set_cell(cursor, CharCell { c: c, attr: attr });
                self.advance_cursor();
            },

            // Local cursor movement
            &CarriageReturn => self.cursor.x = 0,
            &CursorLeft     => self.rewind_cursor(),
            &CursorDown     => self.advance_cursor_line(),
            &CursorRight    => self.advance_cursor(),
            &CursorUp       => self.cursor.y = max(0, self.cursor.y-1),
            &CursorHome     => self.cursor = Position { x:0, y:0 },

            // Parameter local cursor movement
            &CursorPDown(n)  => self.cursor.y = min(self.cursor.y + n, self.h-1),
            &CursorPUp(n)    => self.cursor.y = max(self.cursor.y - n, 0),
            &CursorPRight(n) => self.cursor.x = min(self.cursor.x + n, self.w-1),
            &CursorPLeft(n)  => self.cursor.x = max(self.cursor.x - n, 0),

            // Absolute position
            &SaveCursorPosition    => self.saved_cursor = self.cursor,
            &RestoreCursorPosition => self.cursor = self.saved_cursor,
            &MoveCursor(x,y)       => self.cursor = P(max(0, min(x-1, self.w-1)), max(0, min(y-1, self.h-1))),
            &MoveCursorColumn(x)   => self.cursor.x = max(0, min(x-1, self.w-1)),
            &MoveCursorRow(y)      => self.cursor.y = max(0, min(y-1, self.h-1)),

            // Scrolling
            &ChangeScrollRegion(y1, y2) => { self.scroll_region = (y1-1, y2); self.cursor = P(0,0); }
            &ScrollForward(n)           => self.scroll(n as i16),
            &ScrollReverse(n)           => self.scroll(-(n as i16)),

            // Add to screen
            &InsertLine     => self.insert_lines(1),
            &InsertLines(n) => self.insert_lines(n),

            // Delete from screen
            &ClearScreen    => for pos in &self.all_cell_positions() { self.set_cell(*pos, Default::default()); },
            &DeleteChar     => self.delete_chars(1),
            &DeleteChars(n) => self.delete_chars(n),
            &DeleteLine     => self.delete_lines(1),
            &DeleteLines(n) => self.delete_lines(n),
            &EraseChars(n)  => {
                let y = self.cursor.y;
                for x in self.cursor.x..(self.cursor.x+n) { 
                    self.set_char(P(x, y), ' '); 
                }
            },
            &ClearEOS       => for y in self.cursor.y..self.h { self.clear_line(y); },
            &ClearEOL       => {
                let y = self.cursor.y;
                for x in self.cursor.x..self.w { self.set_char(P(x,y), ' '); }
            },
            &ClearBOL       => {
                let y = self.cursor.y;
                for x in 0..self.cursor.x+1 { self.set_char(P(x,y), ' '); }
            },

            // Insert mode
            &SetInsertMode(b) => self.insert_mode = b,
            &InsertChars(n)   => self.insert_chars(n),

            // Attributes
            &SetStandoutMode(b)  => self.current_attribute.standout = b,
            &SetUnderlineMode(b) => self.current_attribute.underline = b,
        }
    }

    pub fn dirty(&mut self) -> Vec<Position> {
        let drt = self.dirty.clone();
        self.dirty.clear();
        drt
    }

    pub fn update_cursor(&self) {
        // TODO - timing function to blink cursor
    }

    fn set_cell(&mut self, cursor: Position, c: CharCell) {
        if cursor.x < self.w && cursor.y < self.h {
            self.cells.insert(cursor, c);
            self.dirty.push(cursor);
        }
    }

    fn set_char(&mut self, cursor: Position, c: char) {
        let attr = self.current_attribute;
        self.set_cell(cursor, CharCell { c: c, attr: attr });
    }

    fn rewind_cursor(&mut self) {
        if self.cursor.x > 0 {
            self.cursor.x -= 1;
        }
    }

    fn advance_cursor(&mut self) {
        self.cursor.x += 1;
        if self.cursor.x >= self.w {
            self.cursor.x = 0;
            self.advance_cursor_line();
        }
    }

    fn advance_cursor_line(&mut self) {
        self.cursor.y += 1;
        if self.cursor.y >= self.scroll_region.1 {
            self.scroll(1);
            self.cursor.y -= 1;
        }
        if self.cursor.y >= self.h {
            self.cursor.y = self.h - 1;
        }
    }

    fn scroll(&mut self, n: i16) {
        if n > 0 {
            let n = n as u16;
            for y in (self.scroll_region.0+n) .. self.scroll_region.1 {
                self.move_line(y, y-n);
            }
            for y in (self.scroll_region.1 - n) .. self.scroll_region.1 {
                self.clear_line(y);
            }
        } else if n < 0 {
            let n = -n as u16;
            for y in ((self.scroll_region.0+n)..self.scroll_region.1).rev() {
                self.move_line(y-n, y);
            }
            for y in self.scroll_region.0 .. (self.scroll_region.0+n) {
                self.clear_line(y);
            }
        }
    }

    fn move_line(&mut self, y_orig: u16, y_dest: u16) {
        for x in 0..self.w {
            let c = self.cells[&P(x, y_orig)];
            self.set_cell(P(x, y_dest), c);
        }
    }

    fn clear_line(&mut self, y: u16) {
        for x in 0..self.w {
            self.set_char(P(x, y), ' ');
        }
    }

    fn insert_lines(&mut self, n: u16) {
        let y = self.scroll_region.0;
        self.scroll_region.0 = self.cursor.y;
        self.scroll(-(n as i16));
        self.scroll_region.0 = y;
    }

    fn delete_lines(&mut self, n: u16) {
        let y = self.scroll_region.0;
        self.scroll_region.0 = self.cursor.y;
        self.scroll(n as i16);
        self.scroll_region.0 = y;
    }

    fn insert_chars(&mut self, n: u16) {
        let y = self.cursor.y;
        for x in ((self.cursor.x+n)..self.w).rev() {
            let c = self.cells[&P(x-n, y)];
            self.set_cell(P(x,y), c);
        }
        for x in self.cursor.x..(self.cursor.x+n) {
            self.set_char(P(x, y), ' ');
        }
    }

    fn delete_chars(&mut self, n: u16) {
        let y = self.cursor.y;
        for x in self.cursor.x..(self.w - n) {
            let c = self.cells[&P(x+n, y)];
            self.set_cell(P(x, y), c);
        }
        for x in (self.w-n)..self.w {
            self.set_char(P(x, y), ' ');
        }
    }

    fn all_cell_positions(&self) -> Vec<Position> {
        let mut positions: Vec<Position> = Vec::new();
        for (pos,_) in &self.cells { positions.push(*pos); }
        positions
    }
}

// vim: ts=4:sw=4:sts=4:expandtab
