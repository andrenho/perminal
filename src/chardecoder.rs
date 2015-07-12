#[derive(Debug)]
struct CharDecoder;

#[derive(PartialEq, Eq, Debug)]
enum Conversion {
    Complete(char),
    Incomplete,
    Invalid,
}

impl CharDecoder {

    fn new(input: &str, output: &str) -> Self {
        CharDecoder
    }

    fn convert(&self, c: char) -> Conversion {
        Conversion::Invalid
    }

}

impl Drop for CharDecoder {
    fn drop(&mut self) {
    }
}


#[cfg(test)]
mod tests {

    use super::CharDecoder;
    use super::Conversion;

    #[test]
    fn single_char() { 
        assert_eq!(CharDecoder::new("utf-8", "latin1").convert('a'), Conversion::Complete('a')); 
    }

    #[test]
    fn utf8_char() { 
        assert_eq!(CharDecoder::new("utf-8", "latin1").convert('á'), Conversion::Complete(225 as char)); 
    }
}

// vim: ts=4:sw=4:sts=4:expandtab
