use std::collections::HashMap;
use std::cell::RefCell;
use std::str;

use chardecoder::CharDecoder;
use chardecoder::Conversion::*;
use command::Command;
use command::Command::*;
use userevent::UserEvent;
use userevent::UserEvent::KeyPress;
use userevent::UserEvent::SpecialKeyPress;
use userevent::SpecialKey::*;

const MAX_COMMAND_SIZE : usize = 16;

//
// TERMINAL
//
pub struct Terminal {
    cd: CharDecoder,
}

impl Terminal {

    pub fn new() -> Self {
        Terminal {
            cd: CharDecoder::new("utf-8", "utf-8"),
        }
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
        let mut cmds : Vec<Command> = Vec::new();
        let mut cmd_mode = false;
        let mut current_cmd : Vec<u8> = Vec::new();

        while !data.is_empty() {
            
            //for d in data.iter() { print!("{} ", d); } println!("");

            match(self.cd.convert(data.to_vec())) {
                
                Ok(d) => {
                    for _ in d.iter() { data.remove(0); }  // remove from queue
                    match cmd_mode {
                        false => {
                            if d[0] == 27 {
                                cmd_mode = true;
                                current_cmd.push(27);
                            } else {
                                cmds.push(PrintChar(d));
                            }
                        },
                        true => {
                            current_cmd.extend(d.into_iter());
                            if *current_cmd.last().unwrap() != '|' as u8 {
                                // check if the command is too long
                                if current_cmd.len() > MAX_COMMAND_SIZE {
                                    for c in &current_cmd { cmds.push(PrintChar(vec![*c])); }
                                    cmd_mode = false;
                                    current_cmd.clear();
                                }
                            } else {
                                match self.interpret_command(str::from_utf8(&current_cmd).unwrap()) {
                                    NoOp    => {
                                        for c in &current_cmd { cmds.push(PrintChar(vec![*c])); }
                                        cmd_mode = false;
                                        current_cmd.clear();
                                    }
                                    cmd @ _ => {
                                        cmds.push(cmd);
                                        cmd_mode = false;
                                        current_cmd.clear();
                                    },
                                }
                            }
                        },
                    };
                },

                Invalid => { 
                    data.remove(0);   // remove from queue
                    cmds.push(InvalidUtf8);
                },

                Incomplete(d) => { 
                    break;  // exit loop
                }
                
            }
        }
        data.extend(current_cmd.into_iter());
        cmds
    }

    
    fn interpret_command(&self, cmd: &str) -> Command {
        match(cmd) {
            "\x1b@cuf1|" => CursorRight,
            _ => NoOp,
        }
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
        assert_eq!(t.parse_output_from_plugin(&mut data), [PrintChar(vec!['a' as u8]), PrintChar(vec!['b' as u8]), PrintChar(vec!['c' as u8])]);
        assert_eq!(data.len(), 0);

        // utf-8 data
        let mut data = vec![97u8, 195u8, 161u8];
        assert_eq!(t.parse_output_from_plugin(&mut data), [PrintChar(vec![97u8]), PrintChar(vec![195u8, 161u8])]);
        assert_eq!(data.len(), 0);

        // incomplete data
        let mut data = vec!['a' as u8, 195u8];
        assert_eq!(t.parse_output_from_plugin(&mut data), [PrintChar(vec!['a' as u8])]);
        assert_eq!(data.len(), 1);
        assert_eq!(data[0], 195u8);

        // invalid UTF-8
        let mut data = vec![0xc0 as u8];
        assert_eq!(t.parse_output_from_plugin(&mut data), [InvalidUtf8]);
        assert_eq!(data.len(), 0);
    }


    #[test]
    fn output_commands() {
        let t = Terminal::new();

        // complete data
        let mut data = "a\x1b@cuf1|".to_string().into_bytes();
        assert_eq!(t.parse_output_from_plugin(&mut data), [PrintChar(vec!['a' as u8]), CursorRight]);
        assert_eq!(data.len(), 0);

        // incomplete data
        let mut data = "a\x1b@cu".to_string().into_bytes();
        assert_eq!(t.parse_output_from_plugin(&mut data), [PrintChar(vec!['a' as u8])]);
        assert_eq!(data.len(), 4);
        assert_eq!(data, [27u8, '@' as u8, 'c' as u8, 'u' as u8]);

        // command too long
        let mut data = "a\x1b@abcdefghijklmnopqrstuvwxyz".to_string().into_bytes();
        t.parse_output_from_plugin(&mut data);
        assert_eq!(data.len(), 0);

        // not a real command
        let mut data = "a\x1b@x|b".to_string().into_bytes();
        assert_eq!(t.parse_output_from_plugin(&mut data), [
            PrintChar(vec!['a' as u8]), PrintChar(vec![27u8]), PrintChar(vec!['@' as u8]), PrintChar(vec!['x' as u8]), PrintChar(vec!['|' as u8]), PrintChar(vec!['b' as u8])
        ]);
        assert_eq!(data.len(), 0);

        // two commands
        let mut data = "a\x1b@cuf1|\x1b@cuf1|\x1b@cu".to_string().into_bytes();
        assert_eq!(t.parse_output_from_plugin(&mut data), [PrintChar(vec!['a' as u8]), CursorRight, CursorRight]);
        assert_eq!(data.len(), 4);
        assert_eq!(data, [27u8, '@' as u8, 'c' as u8, 'u' as u8]);
    }

    
    #[test]
    fn output_command_parameters() {
        // complete data
        let t = Terminal::new();
        let mut data = "\x1b@csr#12;32|".to_string().into_bytes();
        assert_eq!(t.parse_output_from_plugin(&mut data), [ChangeScrollRegion(12, 32)]);

        // invalid data (should rollback)
        let t = Terminal::new();
        let mut data = "\x1b@csr#12;32a".to_string().into_bytes();
        t.parse_output_from_plugin(&mut data);
        assert_eq!(data.len(), 0);
    }

    // TODO - test other terminals expressions
}

// vim: ts=4:sw=4:sts=4:expandtab
