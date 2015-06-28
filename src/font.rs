use matrix::Attributes;

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct CharImage {
    pub w: u32,
    pub h: u32,
    pub data: Vec<Color>,
}

pub trait Font {
    fn char_height(&self) -> u32;
    fn char_width(&self) -> u32;
    fn load_char(&self, c: char, attr: Attributes) -> CharImage;
}

// vim: ts=4:sw=4:sts=4:expandtab