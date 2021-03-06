use chars::*;
use command::Command;
use command::Command::*;
use chardecoder::*;


//
// MATRIX
//
#[allow(dead_code)]
struct ScrollRegion {
    top: usize,
    bottom: usize,
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Clone)]
pub struct Dirty {
    cells: Vec<Position>,
    full_screen: bool,
    bell: bool,
    flash: bool
}

pub struct Matrix {
    pub w: usize,
    pub h: usize,
    cells: Vec<Vec<Cell>>,
    current_attr: Attributes,
    sr: ScrollRegion,
    cursor: Position,
    dirty: Dirty,
}


impl Matrix {

    pub fn new(w: usize, h: usize) -> Self {
        let current_attr : Attributes = Default::default();

        // create cells
        let mut cells : Vec<Vec<Cell>> = Vec::new();
        for y in 0..h { 
            cells.push(Vec::new());
            for _ in 0..w {
                cells[y].push(Cell { c: ' ', attr: current_attr });
            }
        }

        Matrix { 
            w: w,
            h: h, 
            cells: cells, 
            current_attr: current_attr,
            sr: ScrollRegion { top: 0, bottom: h },
            cursor: Position { x: 0, y: 0 },
            dirty: Dirty {
                cells: Vec::new(),
                full_screen: false,
                bell: false,
                flash: false,
            }
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
            PrintChar(v) => self.print_char(&v),
            _ => (),
        }
    }


    fn print_char(&mut self, v: &Vec<u8>) {
        match CharDecoder::vec_to_char(v) {
            Some(ch) => self.cells[self.cursor.y][self.cursor.x] = Cell { c: ch, attr: self.current_attr },
            None     => self.execute(InvalidUtf8),
        }
        let c = self.cursor;
        self.dirty.cells.push(Position { x: c.x, y: c.y });
        self.cursor_advance_x(1);
    }


    pub fn dirty(&mut self) -> Dirty {
        let d = Dirty {
            cells: self.dirty.cells.to_vec(),
            full_screen: self.dirty.full_screen,
            bell: self.dirty.bell,
            flash: self.dirty.flash,
        };
        self.dirty = Dirty { 
            cells: Vec::new(), 
            full_screen: false,
            bell: false,
            flash: false,
        };
        d
    }


    fn move_cursor(&mut self, x: usize, y: usize) {
        self.cursor.x = x;
        self.cursor.y = y;
    }


    fn cursor_advance_x(&mut self, n: isize) {
        let x = ((self.cursor.x as isize) + n) as usize;
        let y = self.cursor.y;
        if x < self.w {
            self.move_cursor(x, y);
        } else {
            self.cursor.x = 0;
            self.cursor_advance_y();
        }
    }


    fn cursor_advance_y(&mut self) {
        let x = self.cursor.x;
        let y = self.cursor.y+1;
        if y < self.sr.bottom {
            self.move_cursor(x, y);
        } else {
            self.scroll(-1);
        }
    }


    fn scroll(&mut self, n: isize) {
        if n < 0 {
            let n = (-n) as usize;
            for _ in 0 .. n {
                // remove from top
                self.cells.remove(0);
                // add to bottom
                let mut v = Vec::new();
                for _ in 0..self.w {
                    v.push(Cell { c: ' ', attr: self.current_attr });
                }
                self.cells.push(v);
            }
            self.dirty.full_screen = true;
        } else if n > 0 {
            let n = n as usize;
            for _ in 0 .. n {
                // remove from bottom
                self.cells.pop();
                // add to the top
                let mut v = Vec::new();
                for _ in 0..self.w {
                    v.push(Cell { c: ' ', attr: self.current_attr });
                }
                self.cells.insert(0, v);
            }
            self.dirty.full_screen = true;
        }
    }
    
}


//
// TESTS
//
#[cfg(test)]
mod tests {

    use super::Matrix;
    use super::Position;
    use command::Command::*;

    // 
    // TEST COMMANDS
    //

    #[test] 
    fn out_of_bounds() {
        let m = Matrix::new(80, 25);
        assert!(m.cell(80, 0).is_err());
        assert!(m.cell(0, 25).is_err());
        assert!(m.cell(79, 24).is_ok());
    }

    #[test] 
    fn print_char() {
        let mut m = Matrix::new(80, 25);
        m.execute(PrintChar(vec!['a' as u8]));
        assert_eq!(m.cell(0, 0).unwrap().c, 'a');
        assert_eq!(m.cell(1, 0).unwrap().c, ' ');
        m.execute(PrintChar(vec!['a' as u8]));
        assert_eq!(m.cell(1, 0).unwrap().c, 'a');
    }

    #[test]
    fn dirty() {
        let mut m = Matrix::new(80, 25);
        m.execute(PrintChar(vec!['a' as u8]));
        m.execute(PrintChar(vec!['a' as u8]));
        let d = m.dirty();
        assert_eq!(d.cells.len(), 2);
        assert_eq!(d.cells[0], Position { x:0, y:0 });
        assert_eq!(d.cells[1], Position { x:1, y:0 });
        let d2 = m.dirty();
        assert_eq!(d2.cells.len(), 0);
    }

    #[test]
    fn print_char_utf8() {
        let mut m = Matrix::new(80, 25);
        m.execute(PrintChar(vec![195u8, 161u8]));
        assert_eq!(m.cell(0, 0).unwrap().c, 'á');
        assert_eq!(m.cursor.x, 1);
        assert_eq!(m.cursor.y, 0);
    }
    
    #[test]
    fn screen_right_border() {
        let mut m = Matrix::new(80, 25);
        m.move_cursor(79, 0);
        m.execute(PrintChar(vec!['a' as u8]));
        assert_eq!(m.cursor.x, 0);
        assert_eq!(m.cursor.y, 1);
    }

    #[test]
    fn scroll_up() {
        let mut m = Matrix::new(80, 25);
        m.move_cursor(0, 2);
        m.execute(PrintChar(vec!['a' as u8]));
        m.move_cursor(0, 22);
        m.execute(PrintChar(vec!['a' as u8]));
        m.scroll(-6);
        assert_eq!(m.cell(0, 2).unwrap().c, ' ');
        assert_eq!(m.cell(0, 16).unwrap().c, 'a');
        assert_eq!(m.cell(0, 22).unwrap().c, ' ');
    }

    #[test]
    fn scroll_down() {
        let mut m = Matrix::new(80, 25);
        m.execute(PrintChar(vec!['a' as u8]));
        m.move_cursor(0, 2);
        m.execute(PrintChar(vec!['a' as u8]));
        m.scroll(6);
        assert_eq!(m.cell(0, 6).unwrap().c, 'a');
        assert_eq!(m.cell(0, 8).unwrap().c, 'a');
        assert_eq!(m.cell(0, 0).unwrap().c, ' ');
    }

    #[test]
    fn screen_bottom_border() {
        let mut m = Matrix::new(80, 25);
        m.move_cursor(79, 24);
        m.execute(PrintChar(vec!['a' as u8]));
        assert_eq!(m.cell(79,23).unwrap().c, 'a');
        assert_eq!(m.cell(79,24).unwrap().c, ' ');
        assert_eq!(m.cursor.x, 0);
        assert_eq!(m.cursor.y, 24);
    }

    #[test]
    fn screen_fill() {
        let mut m = Matrix::new(80, 25);
        for _ in 0 .. 30000 {
            m.execute(PrintChar(vec!['x' as u8]));
        }
        assert_eq!(m.cursor.y, 24);
    }

}

// vim: ts=4:sw=4:sts=4:expandtab
