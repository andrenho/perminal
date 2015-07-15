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
        enum CommandMode { Regular, Command }

        // this lambda rollbacks data in command buffer back to data
        let rollback = |cmd_buffer: &mut Vec<u8>, cmd_mode: &mut CommandMode, cmds: &mut Vec<Command>| {
            for c in cmd_buffer.iter() { cmds.push(PrintChar(vec![*c])); }
            *cmd_mode = CommandMode::Regular;
            cmd_buffer.clear();
        };

        let mut cmds : Vec<Command> = Vec::new();
        let mut cmd_mode = CommandMode::Regular;
        let mut cmd_buffer : Vec<u8> = Vec::new();

        while !data.is_empty() {
            
            //for d in data.iter() { print!("{} ", d); } println!("");

            match self.cd.convert(data.to_vec()) {
                
                Complete(d) => {
                    for _ in d.iter() { data.remove(0); }  // remove from queue
                    match cmd_mode {

                        CommandMode::Regular => {
                            if d[0] == 27 {
                                cmd_mode = CommandMode::Command;
                                cmd_buffer.push(27);
                            } else {
                                cmds.push(PrintChar(d));
                            }
                        },
                        
                        CommandMode::Command => {

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
                                    Some(cmd) => {
                                        cmds.push(cmd);
                                        cmd_mode = CommandMode::Regular;
                                        cmd_buffer.clear();
                                    },
                                    None => rollback(&mut cmd_buffer, &mut cmd_mode, &mut cmds),
                                }
                            }
                        },
                    };
                },

                Invalid => { 
                    data.remove(0);   // remove from queue
                    cmds.push(InvalidUtf8);
                },

                Incomplete(_) => { 
                    break;  // exit loop
                }
                
            }
        }
        data.extend(cmd_buffer.into_iter());
        cmds
    }

    
    fn interpret_command(&self, cmd: &str) -> Option<Command> {
        let mut p : Vec<u16> = Vec::new();
        match self.interpret_parameters(cmd, &mut p) {
            Some(v) => match v.as_ref() {
                "\x1b@cuf1|"  => Some(CursorRight),
                "\x1b@csr##|" => Some(ChangeScrollRegion(p[0], p[1])),
                _ => None,
            },
            None => None,
        }
    }


    fn interpret_parameters(&self, cmd: &str, p: &mut Vec<u16>) -> Option<String> {
        // check if there's a ';'
        let s = cmd.to_string();
        if !cmd.to_string().contains(';') {
            return Some(s);
        }

        // string contains a ';' - start parsing
        enum Mode { Command, Number }
        let mut mode = Mode::Command;
        let mut command : Vec<char> = Vec::new();
        let mut current : Vec<char> = Vec::new();

        for c in s.chars() {
            match mode {
                Mode::Command => { 
                    if c == ';' {
                        mode = Mode::Number;
                        command.push('#');
                    } else {
                        command.push(c);
                    }
                },
                Mode::Number => {
                    if c == '|' || c == ';' {
                        match current.iter().cloned().collect::<String>().parse::<u16>() {
                            Ok(n)  => p.push(n),
                            Err(_) => return None,
                        }
                        current.clear();
                        if c == '|' {
                            command.push('|');
                            //println!("{:?}", command);
                            return Some(command.into_iter().collect())
                        } else {
                            command.push('#');
                        }
                    } else if !c.is_digit(10) {
                        return None
                    } else {
                        current.push(c);
                    }
                },
            };
        }

        // needs to rollback to data
        None
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
    // TEST PLUGIN Command OUTPUT
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
    // TEST PLUGIN Command PARAMETERS OUTPUT
    //

    #[test] fn cmdpar_complete_data() {
        let t = Terminal::new();
        let mut data = "\x1b@csr;12;32|".to_string().into_bytes();
        assert_eq!(t.parse_output_from_plugin(&mut data), [ChangeScrollRegion(12, 32)]);
    }

    #[test] fn cmdpar_invalid_data() {  // should rollback
        let t = Terminal::new();
        let mut data = "\x1b@csr;12;32a|".to_string().into_bytes();
        assert_eq!(t.parse_output_from_plugin(&mut data), printchar_array("\x1b@csr;12;32a|"));
        assert_eq!(data.len(), 0);
    }

    #[test] fn cmdpar_incomplete_data() {
        let mut data = "\x1b@csr;12;3".to_string().into_bytes();
        assert_eq!(Terminal::new().parse_output_from_plugin(&mut data), []);
        assert_eq!(data.len(), 10);

        data.push('2' as u8); data.push('|' as u8);
        assert_eq!(Terminal::new().parse_output_from_plugin(&mut data), [ChangeScrollRegion(12, 32)]);
        assert_eq!(data.len(), 0);
    }

    #[test] fn cmdpar_not_a_real_command() {
        let mut data = "\x1b@csp;12;32|".to_string().into_bytes();
        let v = Terminal::new().parse_output_from_plugin(&mut data);
        assert_eq!(v.len(), 12);
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
