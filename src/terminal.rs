use std::collections::HashMap;

use chardecoder::CharDecoder;
use command::Command;
use userevent::UserEvent;
use userevent::UserEvent::KeyPress;
use userevent::UserEvent::SpecialKeyPress;
use userevent::SpecialKey::*;

//
// TERMINAL
//
pub struct Terminal;

impl Terminal {

    pub fn new() -> Self {
        Terminal
    }

    pub fn parse_user_event(&self, event: UserEvent) -> Vec<u8> {
        match event {
            KeyPress(c) => c,
            SpecialKeyPress(key) => match key {
                F12 => "@kf12|",
            }.to_string().into_bytes(),
        }
    }

    pub fn parse_output_from_plugin(&self, data: &mut Vec<u8>) -> Vec<Command> {
        unimplemented!()
    }

}


//
// TESTS
//
#[cfg(test)]
mod tests {

    use super::Terminal;
    use userevent::UserEvent::KeyPress;
    use userevent::UserEvent::SpecialKeyPress;
    use userevent::SpecialKey::*;
    use command::Command::*;


    #[test]
    fn user_events() {
        let t = Terminal::new();
        assert_eq!(t.parse_user_event(KeyPress(vec!['a' as u8])), vec!['a' as u8]);
        assert_eq!(t.parse_user_event(KeyPress(vec!['A' as u8])), vec!['A' as u8]);
        assert_eq!(t.parse_user_event(KeyPress(vec![195, 161])), vec![195, 161]);
        assert_eq!(t.parse_user_event(SpecialKeyPress(F12)), "@kf12|".as_bytes());
        // TODO - mouse
    }


    #[test]
    fn output_text() {
        let t = Terminal::new();

        // complete data
        let mut data = "abc".to_string().into_bytes();
        assert_eq!(t.parse_output_from_plugin(&mut data), [PrintChar('a'), PrintChar('b'), PrintChar('c')]);
        assert_eq!(data.len(), 0);
        
        // incomplete data
        let mut data = vec!['a' as u8, 195u8];
        assert_eq!(t.parse_output_from_plugin(&mut data), [PrintChar('a')]);
        assert_eq!(data.len(), 1);
        assert_eq!(data[0], 195u8);

        // invalid UTF-8
        let mut data = vec![0xc0 as u8];
        assert_eq!(t.parse_output_from_plugin(&mut data), [InvalidUtf8(0xc0)]);
        assert_eq!(data.len(), 0);
    }


    #[test]
    fn output_commands() {
        let t = Terminal::new();

        // complete data
        let mut data = "a@cuf1|".to_string().into_bytes();
        assert_eq!(t.parse_output_from_plugin(&mut data), [PrintChar('a'), CursorRight]);
        assert_eq!(data.len(), 0);

        // incomplete data
        let mut data = "a@cu".to_string().into_bytes();
        assert_eq!(t.parse_output_from_plugin(&mut data), [PrintChar('a')]);
        assert_eq!(data.len(), 3);
        assert_eq!(data, ['@' as u8, 'c' as u8, 'u' as u8]);

        // command too long
        let mut data = "a@abcdefghijklmnopqrstuvwxyz".to_string().into_bytes();
        t.parse_output_from_plugin(&mut data);
        assert_eq!(data.len(), 0);

        // not a real command
        let mut data = "a@x|b".to_string().into_bytes();
        assert_eq!(t.parse_output_from_plugin(&mut data), [
            PrintChar('a'), PrintChar('@'), PrintChar('x'), PrintChar('|'), PrintChar('b')
        ]);
        assert_eq!(data.len(), 0);
    }

    
    #[test]
    fn output_command_parameters() {
        // complete data
        let t = Terminal::new();
        let mut data = "@csr#12;32|".to_string().into_bytes();
        assert_eq!(t.parse_output_from_plugin(&mut data), [ChangeScrollRegion(12, 32)]);

        // invalid data (should rollback)
        let t = Terminal::new();
        let mut data = "@csr#12;32a".to_string().into_bytes();
        t.parse_output_from_plugin(&mut data);
        assert_eq!(data.len(), 0);
    }

    // TODO - test other terminals expressions
}

// vim: ts=4:sw=4:sts=4:expandtab
