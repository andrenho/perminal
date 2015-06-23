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
    cursor: (u16, u16),
    dirty: Vec<(u16,u16)>,
}

impl Matrix {
    pub fn new(w: u16, h: u16) -> Matrix {
        let mut m = Matrix {
            w: w,
            h: h,
            cursor: (0, 0),
            cells: HashMap::new(),
            dirty: vec![],
        };
        for x in 0..(w-1) {
            for y in 0..(h-1) {
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
        c
    }

    fn set_char(&mut self, cursor: &(u16, u16), c: char) {
        match self.cells.get_mut(cursor) {
            Some(cell) => { 
                cell.c = c;
                self.dirty.push(*cursor);
            }
            _ => panic!("Invalid cursor value!"),
        }
    }

    fn advance_cursor(&mut self) {
        self.cursor.0 += 1;
    }
}

// vim: ts=4:sw=4:sts=4:expandtab
