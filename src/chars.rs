#[derive(Clone,Copy,PartialEq,Eq,Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}


#[derive(Clone,Copy,PartialEq,Eq,Hash)]
pub struct Attributes {
    pub standout: bool,
    pub underline: bool,
    pub reverse: bool,
    pub blink: bool,
    pub bold: bool,
    pub dim: bool,
    pub invisible: bool,
    pub protected: bool,
    pub acs: bool,
    pub fg_color: Color,
    pub bg_color: Color,
}

impl Default for Attributes {
    fn default() -> Attributes {
        Attributes {
            standout: false,
            underline: false,
            reverse: false,
            blink: false,
            bold: false,
            dim: false,
            invisible: false,
            protected: false,
            acs: false,
	    fg_color: Color { r:0, g:0, b:0 },
	    bg_color: Color { r:255, g:255, b:255 },
        }
    }
}


// vim: ts=4:sw=4:sts=4:expandtab
