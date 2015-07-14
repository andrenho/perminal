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

enum CommandMode { REGULAR, COMMAND }

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
        let mut cmd_mode = CommandMode::REGULAR;
        let mut cmd_buffer : Vec<u8> = Vec::new();

        while !data.is_empty() {
            
            //for d in data.iter() { print!("{} ", d); } println!("");

            match(self.cd.convert(data.to_vec())) {
                
                Ok(d) => {
                    for _ in d.iter() { data.remove(0); }  // remove from queue
                    match cmd_mode {

                        CommandMode::REGULAR => {
                            if d[0] == 27 {
                                cmd_mode = CommandMode::COMMAND;
                                cmd_buffer.push(27);
                            } else {
                                cmds.push(PrintChar(d));
                            }
                        },
                        
                        CommandMode::COMMAND => {
                            // this lambda rollbacks data in command buffer back to data
                            let rollback = |cmd_buffer: &mut Vec<u8>, cmd_mode: &mut CommandMode, cmds: &mut Vec<Command>| {
                                for c in cmd_buffer.iter() { cmds.push(PrintChar(vec![*c])); }
                                *cmd_mode = CommandMode::REGULAR;
                                cmd_buffer.clear();
                            };

                            // if ESC is found when a command is already being parsed, rollback and reset the parsing
                            if *d.last().unwrap() == 27u8 {  
                                for c in cmd_buffer.iter() { cmds.push(PrintChar(vec![*c])); } // rollback
                                cmd_buffer.clear();
                            } 
                            
                            // add to buffer
                            cmd_buffer.extend(d.into_iter());

                            if *cmd_buffer.last().unwrap() != '|' as u8 {   // command is not over
                                // check if the command is too long
                                if cmd_buffer.len() > MAX_COMMAND_SIZE {
                                    rollback(&mut cmd_buffer, &mut cmd_mode, &mut cmds);
                                }
                            } else {
                                match self.interpret_command(str::from_utf8(&cmd_buffer).unwrap()) {
                                    NoOp    => {
                                        rollback(&mut cmd_buffer, &mut cmd_mode, &mut cmds);
                                    }
                                    cmd @ _ => {
                                        cmds.push(cmd);
                                        cmd_mode = CommandMode::REGULAR;
                                        cmd_buffer.clear();
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
        data.extend(cmd_buffer.into_iter());
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
    use command::Command;
    use command::Command::*;

    
    // 
    // TEST USER EVENTS
    //

    #[test] fn userevent_simple_event() {
        assert_eq!(Terminal::new().parse_user_event(KeyPress(vec!['a' as u8])), vec!['a' as u8]);
    }

    #[test] fn userevent_shift_event() {
        assert_eq!(Terminal::new().parse_user_event(KeyPress(vec!['A' as u8])), vec!['A' as u8]);
    }

    #[test] fn userevent_unicode_event() {
        assert_eq!(Terminal::new().parse_user_event(KeyPress(vec![195, 161])), vec![195, 161]);
    }

    #[test] fn userevent_special_key_event() {
        assert_eq!(Terminal::new().parse_user_event(SpecialKeyPress(F12)), "@kf12|".as_bytes());
    }

    // TODO - mouse


    //
    // TEST PLUGIN SIMPLE OUTPUT
    //
    
    #[test] fn plugin_complete_data() {
        let mut data = "abc".to_string().into_bytes();
        assert_eq!(Terminal::new().parse_output_from_plugin(&mut data), printchar_array("abc"));
        assert_eq!(data.len(), 0);
    }

    #[test] fn plugin_unicode_data() {
        let mut data = vec![97u8, 195u8, 161u8];
        assert_eq!(Terminal::new().parse_output_from_plugin(&mut data), [PrintChar(vec![97u8]), PrintChar(vec![195u8, 161u8])]);
        assert_eq!(data.len(), 0);
    }

    #[test] fn plugin_incomplete_data() {
        let mut data = vec!['a' as u8, 195u8];
        assert_eq!(Terminal::new().parse_output_from_plugin(&mut data), [PrintChar(vec!['a' as u8])]);
        assert_eq!(data.len(), 1);
        assert_eq!(data[0], 195u8);
    }

    #[test] fn plugin_invalid_unicode() {
        let mut data = vec![0xc0 as u8];
        assert_eq!(Terminal::new().parse_output_from_plugin(&mut data), [InvalidUtf8]);
        assert_eq!(data.len(), 0);
    }

    // 
    // TEST PLUGIN COMMAND OUTPUT
    //

    #[test] fn cmd_complete_data() {
        let mut data = "a\x1b@cuf1|".to_string().into_bytes();
        assert_eq!(Terminal::new().parse_output_from_plugin(&mut data), [PrintChar(vec!['a' as u8]), CursorRight]);
        assert_eq!(data.len(), 0);
    }

    #[test] fn cmd_incomplete_data() {
        let mut data = "a\x1b@cu".to_string().into_bytes();
        assert_eq!(Terminal::new().parse_output_from_plugin(&mut data), [PrintChar(vec!['a' as u8])]);
        assert_eq!(data.len(), 4);
        assert_eq!(data, [27u8, '@' as u8, 'c' as u8, 'u' as u8]);

        data.push('f' as u8); data.push('1' as u8); data.push('|' as u8);
        assert_eq!(Terminal::new().parse_output_from_plugin(&mut data), [CursorRight]);
        assert_eq!(data.len(), 0);
    }

    #[test] fn cmd_command_too_long() {
        let mut data = "a\x1b@abcdefghijklmnopqrstuvwxyz".to_string().into_bytes();
        Terminal::new().parse_output_from_plugin(&mut data);
        assert_eq!(data.len(), 0);
    }

    #[test] fn cmd_not_a_real_command() {
        let mut data = "a\x1b@x|b".to_string().into_bytes();
        assert_eq!(Terminal::new().parse_output_from_plugin(&mut data), printchar_array("a\x1b@x|b"));
        assert_eq!(data.len(), 0);
    }

    #[test] fn cmd_two_commands() {
        let mut data = "a\x1b@cuf1|\x1b@cuf1|\x1b@cu".to_string().into_bytes();
        assert_eq!(Terminal::new().parse_output_from_plugin(&mut data), [PrintChar(vec!['a' as u8]), CursorRight, CursorRight]);
        assert_eq!(data.len(), 4);
        assert_eq!(data, [27u8, '@' as u8, 'c' as u8, 'u' as u8]);
    }

    #[test] fn cmd_esc_in_midcommand() {
        let mut data = "\x1b@cu\x1b@cuf1|".to_string().into_bytes();
        assert_eq!(Terminal::new().parse_output_from_plugin(&mut data), [
            PrintChar(vec![27u8]), PrintChar(vec!['@' as u8]), PrintChar(vec!['c' as u8]), 
            PrintChar(vec!['u' as u8]), CursorRight
        ]);
        assert_eq!(data.len(), 0);
    }


    // 
    // TEST PLUGIN COMMAND PARAMETERS OUTPUT
    //

    #[test] fn cmdpar_complete_data() {
        let t = Terminal::new();
        let mut data = "\x1b@csr#12;32|".to_string().into_bytes();
        assert_eq!(t.parse_output_from_plugin(&mut data), [ChangeScrollRegion(12, 32)]);
    }

    #[test] fn cmdpar_invalid_data() {  // should rollback
        let t = Terminal::new();
        let mut data = "\x1b@csr#12;32a".to_string().into_bytes();
        t.parse_output_from_plugin(&mut data);
        assert_eq!(data.len(), 0);
    }

    #[test] fn cmdpar_incomplete_data() {
        unimplemented!()
    }

    #[test] fn cmdpar_not_a_real_command() {
        unimplemented!()
    }


    //
    // HELPER_FUNCTIONS (for tests)
    //

    fn printchar_array(s: &str) -> Vec<Command> {
        let mut v : Vec<Command> = Vec::new();
        for c in s.chars() {
            v.push(PrintChar(vec![c as u8]));
        }
        v
    }

}

// vim: ts=4:sw=4:sts=4:expandtab
