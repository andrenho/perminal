extern crate bmp;
use self::bmp::*;

use font::*;
use chars::Attributes;
use chars::Color;

pub struct BitmapFont {
    image: Image,
    fg: Pixel,
    bg: Pixel,
}

impl BitmapFont {
    pub fn new(filename: &str) -> Self {
        let img = bmp::open(filename).unwrap_or_else(|e| {
            panic!("Failed to open: {}", e)
        });
        BitmapFont {
            image: img,
            fg: img.get_pixel(0, 0),
            bg: img.get_pixel(0, 0),
        }
    }
}

impl Font for BitmapFont {

    fn char_height(&self) -> u32 { 
        self.image.get_height() / 8 
    }
    
    fn char_width(&self) -> u32 { 
        self.image.get_width() / 8 
    }
    
    fn load_char(&self, c: char, attr: Attributes) -> CharImage { 
        let mut data: Vec<Color> = Vec::with_capacity((self.char_height() * self.char_width()) as usize);
        for y in 0..self.char_height() {
            for x in 0..self.char_width() {
                let p = self.image.get_pixel(x, y);
                data.push(Color { r: p.r, g: p.g, b: p.b });
            }
        }
        CharImage { w: self.char_height(), h: self.char_width(), data: data }
    }
}

// vim: ts=4:sw=4:sts=4:expandtab
