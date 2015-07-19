use chardecoder::CharDecoder;
use chars::Color;

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
    cd:           CharDecoder,
}

impl BitmapFont {
    
    pub fn FromXBM(w: usize, h: usize, data: &[u8], encoding: &'static str) -> Self {
        BitmapFont {
            char_width:   w/16,
            char_height:  h/16,
            image_width:  w,
            image_height: h,
            cd:           CharDecoder::new("utf-8", encoding),
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
        let bf = BitmapFont::FromXBM(LATIN1_WIDTH, LATIN1_HEIGHT, &LATIN1_BITS, "latin1");
    }

}

// vim: ts=4:sw=4:sts=4:expandtab
