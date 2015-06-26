use command::Command;
use command::Command::*;
use std::cmp;
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
pub struct Attributes;
impl Default for Attributes {
    fn default() -> Attributes { Attributes }
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
    dirty: Vec<Position>,
    current_attribute: Attributes,
}

impl Matrix {
    pub fn new(w: u16, h: u16) -> Matrix {
        let mut m = Matrix {
            w: w,
            h: h,
            cells: HashMap::new(),
            cursor: Position { x:0, y:0 },
            cursor_on: true,
            dirty: vec![],
            current_attribute: Attributes,
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
                self.set_cell(cursor, CharCell { c: c, attr: attr });
                self.advance_cursor();
            },

            // Local cursor movement
            &CarriageReturn => self.cursor.x = 0,
            &CursorLeft     => self.rewind_cursor(),
            &CursorDown     => self.advance_cursor_line(),
            &CursorRight    => self.advance_cursor(),
            &CursorUp       => self.cursor.y = cmp::max(0, self.cursor.y-1),
            &CursorHome     => self.cursor = Position { x:0, y:0 },

            // Parameter local cursor movement
            &CursorPDown(n) => self.cursor.y = cmp::min(self.cursor.y + n, self.h-1),

            &ClearScreen => { 
                for pos in &self.all_cell_positions() { 
                    self.set_cell(*pos, Default::default()); 
                } 
            },
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
        self.cells.insert(cursor, c);
        self.dirty.push(cursor);
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
        if self.cursor.y >= self.h {
            self.scroll_up();
        }
    }

    fn scroll_up(&mut self) {
        for y in 1..self.h {
            for x in 0..self.w {
                let c = self.cells[&P(x,y)];
                self.set_cell(P(x,y-1), c);
            }
        }
        let h = self.h;
        let c = CharCell { c: ' ', attr: self.current_attribute };
        for x in 0..self.w {
            self.set_cell(P(x,h-1), c);
        }
        self.cursor.y -= 1;
    }

    fn all_cell_positions(&self) -> Vec<Position> {
        let mut positions: Vec<Position> = Vec::new();
        for (pos,_) in &self.cells { positions.push(*pos); }
        positions
    }
}

// vim: ts=4:sw=4:sts=4:expandtab
