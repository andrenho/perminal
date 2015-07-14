extern crate libc;

use std::char;
use std::io::Error;
use std::mem;
use std::ffi::CString;
use std::cmp;

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
pub enum Conversion {
    Complete(Vec<u8>),
    Incomplete(Vec<u8>),
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


    pub fn convert(&self, c: Vec<u8>) -> Conversion {

        if c[0] < 0xbf { return Conversion::Complete(vec![c[0]]); }  // skip single-char conversions

        let inbytesleft = cmp::min(c.len(), 4);
        let outbytesleft : size_t = 4;
        let mut from = c.to_owned();
        let mut to: Vec<u8> = vec![0u8, 0u8, 0u8, 0u8];
        let ret = unsafe{ iconv(self.cd, 
            mem::transmute(&from.as_mut_ptr()), mem::transmute(&inbytesleft),
            mem::transmute(&to.as_mut_ptr()), mem::transmute(&outbytesleft))
        };
        if ret as isize == -1 {
            match Error::last_os_error().raw_os_error().unwrap() {
                E2BIG  => panic!("Invalid outbuf size in iconv."),
                EILSEQ => Conversion::Invalid,
                EINVAL => Conversion::Incomplete(from),
                _      => unreachable!(),
            }
        } else {
            let mut v : Vec<u8> = Vec::new();
            let mut i = 0;
            while to[i] != 0 {
                v.push(to[i]);
                i += 1;
            }
            Conversion::Complete(v)
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
        assert_eq!(cd.convert(vec!['a' as u8]), Conversion::Complete(vec!['a' as u8])); 
    }

    #[test]
    fn utf8_complete_char() { 
        let cd = CharDecoder::new("utf-8", "latin1");
        assert_eq!(cd.convert(vec![195u8, 161u8]), Conversion::Complete(vec![225u8])); 
    }

    #[test]
    fn utf8_incomplete_char() {
        let cd = CharDecoder::new("utf-8", "latin1");
        assert_eq!(cd.convert(vec![195u8]), Conversion::Incomplete(vec![195u8]));
    }

    #[test]
    fn utf8_invalid_char() {
        let cd = CharDecoder::new("utf-8", "latin1");
        assert_eq!(cd.convert(vec![0xc0]), Conversion::Invalid);
    }

    // TODO - for tests, use <http://www.cl.cam.ac.uk/~mgk25/ucs/examples/UTF-8-test.txt>
}

// vim: ts=4:sw=4:sts=4:expandtab
