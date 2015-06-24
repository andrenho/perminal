use command::Command;
use command::Command::*;
use std::collections::HashMap;

#[derive(Clone,Copy)]
pub struct Attributes;

#[derive(Clone,Copy)]
pub struct CharCell {
    pub c: char,
    pub attr: Attributes,
}

pub struct Matrix {
    pub w: u16,
    pub h: u16,
    pub cells: HashMap<(u16,u16), CharCell>,
    pub cursor_on: bool,
    pub cursor: (u16, u16),
    dirty: Vec<(u16,u16)>,
    current_attribute: Attributes,
}

impl Matrix {
    pub fn new(w: u16, h: u16) -> Matrix {
        let mut m = Matrix {
            w: w,
            h: h,
            cells: HashMap::new(),
            cursor: (0, 0),
            cursor_on: true,
            dirty: vec![],
            current_attribute: Attributes,
        };
        for x in 0..w {
            for y in 0..h {
                m.cells.insert((x,y), CharCell { c: ' ', attr: m.current_attribute.clone() });
            }
        }
        m
    }

    pub fn execute(&mut self, cmd: Command) {
        match cmd {
            PrintChar(c) => {
                let cursor = self.cursor;
                let attr = self.current_attribute;
                self.set_char(&cursor, CharCell { c: c, attr: attr.clone() });
                self.advance_cursor();
            },
            LineFeed => { self.advance_cursor_line(); }
            CarriageReturn => { self.cursor.0 = 0; }
        }
    }

    pub fn dirty(&mut self) -> Vec<(u16, u16)> {
        let drt = self.dirty.clone();
        self.dirty.clear();
        drt
    }

    pub fn update_cursor(&self) {
        // TODO - timing function to blink cursor
    }

    fn set_char(&mut self, cursor: &(u16, u16), c: CharCell) {
        self.cells.insert(*cursor, c);
        self.dirty.push(*cursor);
    }

    fn advance_cursor(&mut self) {
        self.cursor.0 += 1;
        if self.cursor.0 >= self.w {
            self.cursor.0 = 0;
            self.advance_cursor_line();
        }
    }

    fn advance_cursor_line(&mut self) {
        self.cursor.1 += 1;
        if self.cursor.1 >= self.h {
            self.scroll_up();
        }
    }

    fn scroll_up(&mut self) {
        for y in 1..self.h {
            for x in 0..self.w {
                let c = self.cells[&(x,y)];
                self.set_char(&(x,y-1), c);
            }
        }
        let h = self.h;
        let c = CharCell { c: ' ', attr: self.current_attribute };
        for x in 0..self.w {
            self.set_char(&(x,h-1), c);
        }
        self.cursor.1 -= 1;
    }
}

// vim: ts=4:sw=4:sts=4:expandtab
