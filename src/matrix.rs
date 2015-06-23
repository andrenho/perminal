use command::Command;
use command::Command::*;
use std::collections::HashMap;

pub struct CharCell {
    pub c: char,
}

pub struct Matrix {
    pub w: u16,
    pub h: u16,
    pub cells: HashMap<(u16,u16), CharCell>,
    pub cursor_on: bool,
    pub cursor: (u16, u16),
    dirty: Vec<(u16,u16)>,
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
        };
        for x in 0..w {
            for y in 0..h {
                m.cells.insert((x,y), CharCell { c: ' ' });
            }
        }
        m
    }

    pub fn execute(&mut self, cmd: Command) {
        match cmd {
            PrintChar(c) => {
                let cursor = self.cursor;
                self.set_char(&cursor, c);
                self.advance_cursor();
            },
        }
    }

    pub fn dirty(&mut self) -> Vec<(u16, u16)> {
        let c = self.dirty.clone();
        self.dirty.clear();
        c
    }

    pub fn update_cursor(&self) {
        // TODO - timing function to blink cursor
    }

    fn set_char(&mut self, cursor: &(u16, u16), c: char) {
        match self.cells.get_mut(cursor) {
            Some(cell) => { 
                cell.c = c;
                self.dirty.push(*cursor);
            }
            _ => unreachable!()
        }
    }

    fn advance_cursor(&mut self) -> (u16, u16) {
        self.cursor.0 += 1;
        if self.cursor.0 >= self.w {
            self.cursor.0 = 0;
            self.cursor.1 += 1;
        }
        if self.cursor.1 >= self.h {
            unimplemented!();  // TODO - skip one line
        }
        (self.cursor.0, self.cursor.1)
    }
}

// vim: ts=4:sw=4:sts=4:expandtab
