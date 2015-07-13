extern crate libc;

use std::char;
use std::io::Error;
use std::mem;
use std::ffi::CString;

use self::libc::consts::os::posix88::{E2BIG, EILSEQ, EINVAL};
use self::libc::{c_char, size_t, c_int, c_void};

//
// C expressions
//
#[allow(non_camel_case_types)]
type iconv_t = *mut c_void;

#[link(name = "util")]
extern "C" {
    fn iconv_open(__tocode: *const c_char, __fromcode: *const c_char) -> iconv_t;
    fn iconv(__cd: iconv_t, __inbuf: *mut *mut c_char,
                 __inbytesleft: *mut size_t, __outbuf: *mut *mut c_char,
                 __outbytesleft: *mut size_t) -> size_t;
    fn iconv_close(__cd: iconv_t) -> c_int;
}


//
// return values from `convert`
//
#[derive(PartialEq, Eq, Debug)]
enum Conversion {
    Ok(char),
    Incomplete,
    Invalid,
}


// 
// char decoder
//
#[derive(Debug)]
pub struct CharDecoder {
    cd: iconv_t,
}


impl CharDecoder {

    pub fn new(input: &str, output: &str) -> Self {
        let from = CString::new(input).unwrap();
        let to   = CString::new(output).unwrap();
        let handle = unsafe { iconv_open(to.as_ptr(), from.as_ptr()) };

        if handle as isize == -1 {
            panic!("Error creating conversion descriptor from {:} to {:}", input, output);
        }

        CharDecoder { cd: handle }
    }


    pub fn convert(&self, c: char) -> Conversion {

        if (c as u32) < 0xbf { return Conversion::Ok(c); }  // skip single-char conversions

        let inbytesleft = match c as u32 {
            0x00000 ... 0x0000ff => 1,
            0x00100 ... 0x00ffff => 2,
            0x10000 ... 0xffffff => 3,
            _                    => 4,
        } as size_t;
        let outbytesleft : size_t = 4;
        let mut from = CharDecoder::decompose(c);
        let mut to = vec![0, 0, 0, 0];
        let ret = unsafe{ iconv(self.cd, 
            mem::transmute(&from.as_mut_ptr()), mem::transmute(&inbytesleft),
            mem::transmute(&to.as_mut_ptr()), mem::transmute(&outbytesleft))
        };
        if ret as isize == -1 {
            match Error::last_os_error().raw_os_error().unwrap() {
                E2BIG  => panic!("Invalid outbuf size in iconv."),
                EILSEQ => Conversion::Invalid,
                EINVAL => Conversion::Incomplete,
                _      => unreachable!(),
            }
        } else {
            let i: u32 = (to[0] as u32) | ((to[1] as u32) << 8) | ((to[2] as u32) << 16) | ((to[3] as u32) << 24);
            match char::from_u32(i) {
                None    => Conversion::Invalid,
                Some(c) => Conversion::Ok(c)
            }
        }
    }

    
    pub fn decompose(c: char) -> Vec<u8> {
        match c as u32 {
            0x00000 ... 0x0000ff => vec![((c as u32) & 0xff) as u8],
            0x00100 ... 0x00ffff => vec![(((c as u32) >> 8) & 0xff) as u8, ((c as u32) & 0xff) as u8],
            0x10000 ... 0xffffff => vec![(((c as u32) >> 16) & 0xff) as u8, (((c as u32) >> 8) & 0xff) as u8, ((c as u32) & 0xff) as u8],
            _                    => vec![(((c as u32) >> 24) & 0xff) as u8, (((c as u32) >> 16) & 0xff) as u8, (((c as u32) >> 8) & 0xff) as u8, ((c as u32) & 0xff) as u8],
        }
    }


}


impl Drop for CharDecoder {
    fn drop(&mut self) {
        unsafe { iconv_close(self.cd); }
    }
}


//
// TESTS
//
#[cfg(test)]
mod tests {

    use std::char;

    use super::CharDecoder;
    use super::Conversion;

    #[test]
    fn single_char() { 
        let cd = CharDecoder::new("utf-8", "latin1");
        assert_eq!(cd.convert('a'), Conversion::Ok('a')); 
    }

    #[test]
    fn utf8_complete_char() { 
        let cd = CharDecoder::new("utf-8", "latin1");
        let chr = char::from_u32((195 << 8) + 161).unwrap();
        assert_eq!(cd.convert(chr), Conversion::Ok(225 as char)); 
    }

    #[test]
    fn utf8_incomplete_char() {
        let cd = CharDecoder::new("utf-8", "latin1");
        assert_eq!(cd.convert(195 as char), Conversion::Incomplete);
    }

    #[test]
    fn utf8_invalid_char() {
        let cd = CharDecoder::new("utf-8", "latin1");
        let chr = 0xc0 as char;
        assert_eq!(cd.convert(chr), Conversion::Invalid);
    }

    #[test]
    fn decompose() {
        assert_eq!(CharDecoder::decompose(char::from_u32((195 << 8) + 161).unwrap()), vec![195, 161]);
    }

    // TODO - for tests, use <http://www.cl.cam.ac.uk/~mgk25/ucs/examples/UTF-8-test.txt>
}

// vim: ts=4:sw=4:sts=4:expandtab
