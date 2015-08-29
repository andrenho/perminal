use chardecoder::CharDecoder;
use chars::Color;
use chars::Attributes;

#[allow(dead_code)]
pub struct CharImage {
    w:    usize,
    h:    usize,
    data: Vec<Color>,
    bg:   Color,
}

#[allow(dead_code)]
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
        // TODO - convert to latin1
        let ch = c as usize;
        let x_in_image = (ch % 16) * self.char_width;
        let y_in_image = (ch / 16) * self.char_height;

        // create base image
        let mut px_image: Vec<u8> = Vec::with_capacity(self.char_width * self.char_height);
        for y in y_in_image..(y_in_image + self.char_height) {
            for x in x_in_image..(x_in_image + self.char_width) {
                px_image.push(self.data[(y * self.image_width) + x]);
            }
        }

        // apply attributes
        if attr.italic                   { self.apply_italic(&mut px_image); }
        if attr.bold                     { self.apply_bold(&mut px_image); }
        if attr.underline                { self.apply_underline(&mut px_image); }
        if attr.dim                      { self.apply_dim(&mut px_image); }
        if attr.standout || attr.reverse { self.apply_reverse(&mut px_image); }
        if attr.invisible                { self.apply_invisible(&mut px_image); }

        // apply base image
        let mut data: Vec<Color> = Vec::with_capacity(self.char_width * self.char_height);
        for px in px_image {
            let ppx = (px as f64) / 255.0;
            data.push(Color {
                r: ((attr.bg_color.r as isize) + ((((attr.fg_color.r as f64) - (attr.bg_color.r as f64)) * ppx) as isize)) as u8,
                g: ((attr.bg_color.g as isize) + ((((attr.fg_color.g as f64) - (attr.bg_color.g as f64)) * ppx) as isize)) as u8,
                b: ((attr.bg_color.b as isize) + ((((attr.fg_color.b as f64) - (attr.bg_color.b as f64)) * ppx) as isize)) as u8,
            });
        }
        CharImage {
            w: self.char_width,
            h: self.char_height,
            data: data,
            bg: attr.bg_color,
        }
    }


    fn apply_italic(&self, px_image: &mut Vec<u8>) {
        for y in 0..(self.char_height) {
            for x in (1..(self.char_width)).rev() {
                let p = x + (y * self.char_width);
                px_image[p] = px_image[p-1];
            }
        }
        for y in 0..(self.char_height) {
            for x in 0..(self.char_width-1) {
                let p = x + (y * self.char_width);
                px_image[p] = px_image[p+1];
            }
        }
    }


    fn apply_bold(&self, px_image: &mut Vec<u8>) {
        for y in 0..(self.char_height) {
            for x in (1..(self.char_width+1)).rev() {
                let p = x + (y * self.char_width);
                if px_image[p-1] > 64 {  // TODO - how much?
                    px_image[p] = px_image[p-1];
                }
            }
        }
    }


    fn apply_underline(&self, px_image: &mut Vec<u8>) {
        #[allow(non_upper_case_globals)]
        const config_underline_y: usize = 2;         // TODO - config
        #[allow(non_upper_case_globals)]
        const config_underline_intensity: u8 = 255;  // TODO - config
        for x in 0..(self.char_width-1) {
            let p = x + ((self.char_height - config_underline_y) * self.char_width);
            px_image[p] = config_underline_intensity;
        }
    }

    fn apply_dim(&self, px_image: &mut Vec<u8>) {
        #[allow(non_upper_case_globals)]
        const config_dim_percentage: f64 = 0.5;  // TODO - config
        for px in px_image.iter_mut() {
            *px = ((*px as f64) * config_dim_percentage) as u8;
        }
    }

    fn apply_reverse(&self, px_image: &mut Vec<u8>) {
        for px in px_image.iter_mut() {
            *px = !*px;
        }
    }

    fn apply_invisible(&self, px_image: &mut Vec<u8>) {
        for px in px_image.iter_mut() {
            *px = 0;
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
    use chardecoder::CharDecoder;
    use chars::Attributes;

    #[test]
    fn load_xbm_font() { 
        let bf = BitmapFont::from_xbm(LATIN1_WIDTH, LATIN1_HEIGHT, &LATIN1_BITS, "latin1");
        let px = |x:usize, y:usize| x + y * LATIN1_WIDTH;
        assert_eq!(bf.data[px(14, 48)], 0);
        assert_eq!(bf.data[px(15, 48)], 255);
        assert_eq!(bf.data[px(165, 343)], 0);
        assert_eq!(bf.data[px(166, 343)], 255);
    }

    #[test]
    fn load_xbm_char() {
        let bf = BitmapFont::from_xbm(LATIN1_WIDTH, LATIN1_HEIGHT, &LATIN1_BITS, "latin1");
        let px = |x:usize, y:usize| x + y * bf.char_width;
        let attr = Default::default();
        let a = bf.load_char('A', &attr);
        assert_eq!(a.data[px(3, 5)], attr.bg_color);
        assert_eq!(a.data[px(4, 5)], attr.fg_color);
    }

    #[test]
    fn load_xbm_utf8() {
        let bf = BitmapFont::from_xbm(LATIN1_WIDTH, LATIN1_HEIGHT, &LATIN1_BITS, "latin1");
        let px = |x:usize, y:usize| x + y * bf.char_width;
        let attr = Default::default();
        let a = bf.load_char(CharDecoder::vec_to_char(&vec![0xc3, 0x81]).unwrap(), &attr);
        assert_eq!(a.data[px(4, 1)], attr.bg_color);
        assert_eq!(a.data[px(5, 1)], attr.fg_color);
    }

    #[test]
    fn attributes() {
        let bf = BitmapFont::from_xbm(LATIN1_WIDTH, LATIN1_HEIGHT, &LATIN1_BITS, "latin1");
        let px = |x:usize, y:usize| x + y * bf.char_width;
        let attr = Default::default();
        let a = bf.load_char('A', &attr);

        let mut attr: Attributes = Default::default(); attr.bold = true;
        let a_bold = bf.load_char('A', &attr);
        assert!(a.data != a_bold.data);

        let mut attr: Attributes = Default::default(); attr.italic = true;
        let a_italic = bf.load_char('A', &attr);
        assert!(a.data != a_italic.data);
        assert!(a.data != a_italic.data);

        let mut attr: Attributes = Default::default(); attr.underline = true;
        let a_und = bf.load_char('A', &attr);
        assert_eq!(a_und.data[px(0, bf.char_height - 2)], attr.fg_color);
        assert!(a.data != a_und.data);

        let mut attr: Attributes = Default::default(); attr.dim = true;
        let a_dim = bf.load_char('A', &attr);
        assert_eq!(a_dim.data[px(4, 5)].r, 128u8);
        assert!(a.data != a_dim.data);

        let mut attr: Attributes = Default::default(); attr.reverse = true;
        let a_rev = bf.load_char('A', &attr);
        assert_eq!(a_rev.data[px(4, 5)], attr.bg_color);
        assert!(a.data != a_rev.data);

        let mut attr: Attributes = Default::default(); attr.invisible = true;
        let a_inv = bf.load_char('A', &attr);
        assert_eq!(a_inv.data[px(4, 5)], attr.bg_color);
        assert!(a.data != a_inv.data);
    }
}

// vim: ts=4:sw=4:sts=4:expandtab
