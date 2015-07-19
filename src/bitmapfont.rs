use chardecoder::CharDecoder;
use chars::Color;
use chars::Attributes;

pub struct CharImage {
    w: usize,
    h: usize,
    px: Vec<Color>,
    bg: Color,
}

pub struct BitmapFont {
    char_width:   usize,
    char_height:  usize,
    image_width:  usize,
    image_height: usize,
    data:         Vec<u8>,
    cd:           CharDecoder,
}

impl BitmapFont {
    
    pub fn from_xbm(w: usize, h: usize, data: &[u8], encoding: &'static str) -> Self {
        let mut new_data: Vec<u8> = Vec::with_capacity(w * h);
        let mut c = 0;
        let bg = data[0] & 1;
        for y in 0..h {
            for k in 0..(w/8) {
                let px = data[(y*(w/8))+k];
                for i in 0..8 {
                    new_data.push(if ((px >> i) & 1) == bg { 0 } else { 255 });
                    c += 1;
                }
            }
        }
        assert!(c == (w*h));
        BitmapFont {
            char_width:   w/16,
            char_height:  h/16,
            image_width:  w,
            image_height: h,
            data:         new_data,
            cd:           CharDecoder::new("utf-8", encoding),
        }
    }

    pub fn load_char(&self, c: char, attr: &Attributes) -> CharImage {
        CharImage {
            w: self.char_width,
            h: self.char_height,
            px: vec![],
            bg: attr.bg_color,
        }
    }

}

//
// TESTS
//
#[cfg(test)]
mod tests {

    use super::BitmapFont;
    use latin1::*;

    #[test]
    fn load_xbm_font() { 
        let bf = BitmapFont::from_xbm(LATIN1_WIDTH, LATIN1_HEIGHT, &LATIN1_BITS, "latin1");
        let px = |x:usize, y:usize| x + y * LATIN1_WIDTH;
        assert_eq!(bf.data[px(14, 48)], 0);
        assert_eq!(bf.data[px(15, 48)], 255);
        assert_eq!(bf.data[px(165, 343)], 0);
        assert_eq!(bf.data[px(166, 343)], 255);
    }

}

// vim: ts=4:sw=4:sts=4:expandtab
