struct CharDecoder;

enum Conversion {
    Complete(char),
    Incomplete,
    Invalid,
}

impl CharDecoder {

    fn new(input: &str, output: &str) -> Self {
        CharDecoder
    }

    fn convert(c: char) -> Conversion {
        Conversion::Invalid
    }

}

impl Drop for CharDecoder {
    fn drop(&mut self) {
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert!(false)
    }
}

// vim: ts=4:sw=4:sts=4:expandtab
