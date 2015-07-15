use std::char;

use chars::*;
use command::Command;
use command::Command::*;


//
// MATRIX
//
struct ScrollRegion {
    top: usize,
    bottom: usize,
}

struct Cursor {
    x: usize,
    y: usize,
}

pub struct Matrix {
    pub w: usize,
    pub h: usize,
    cells: Vec<Vec<Cell>>,
    current_attr: Attributes,
    sr: ScrollRegion,
    cursor: Cursor,
}


impl Matrix {

    pub fn new(w: usize, h: usize) -> Self {
        let current_attr : Attributes = Default::default();

        // create cells
        let mut cells : Vec<Vec<Cell>> = Vec::new();
        for y in 0..h { 
            cells.push(Vec::new());
            for x in 0..w {
                cells[y].push(Cell { c: ' ', attr: current_attr });
            }
        }

        Matrix { 
            w: w,
            h: h, 
            cells: cells, 
            current_attr: current_attr,
            sr: ScrollRegion { top: 0, bottom: h },
            cursor: Cursor { x: 0, y: 0 },
        }
    }


    pub fn cell(&self, x: usize, y: usize) -> Result<Cell, &'static str> {
        if x >= self.w || y >= self.h {
            Err("Cell out of screen bounds")
        } else {
            Ok(self.cells[y][x])
        }
    }


    pub fn execute(&mut self, cmd: Command) {
        match cmd {
            PrintChar(v) => {
                // transform Vec<u8> in u32
                let mut i = -8;
                let cn = v.iter().fold(0, |acc, &item| {
                    i += 8;
                    acc | ((item as u32) << i)
                });
                println!("{}", cn);

                match char::from_u32(cn) {
                    Some(ch) => self.cells[self.cursor.y][self.cursor.x] = Cell { c: ch, attr: self.current_attr },
                    None     => self.execute(InvalidUtf8),
                }
                self.cursor_advance_x(1);
            }
            _ => (),
        }
    }


    fn cursor_advance_x(&mut self, n: isize) {
        self.cursor.x += 1;
        // TODO
    }


}


//
// TESTS
//
#[cfg(test)]
mod tests {

    use super::Matrix;
    use command::Command::*;

    // 
    // TEST COMMANDS
    //

    #[test] fn out_of_bounds() {
        let mut m = Matrix::new(80, 25);
        assert!(m.cell(80, 0).is_err());
        assert!(m.cell(0, 25).is_err());
        assert!(m.cell(79, 24).is_ok());
    }

    #[test] fn print_char() {
        let mut m = Matrix::new(80, 25);
        m.execute(PrintChar(vec!['a' as u8]));
        assert_eq!(m.cell(0, 0).unwrap().c, 'a');
        assert_eq!(m.cell(0, 1).unwrap().c, ' ');
        m.execute(PrintChar(vec!['a' as u8]));
        assert_eq!(m.cell(0, 1).unwrap().c, 'a');
    }

}

// vim: ts=4:sw=4:sts=4:expandtab
