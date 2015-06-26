use terminfo::Terminfo;
use command::Command;
use command::Command::*;
use userevent::Key;
use userevent::Key::*;

pub struct TerminfoXterm256 {
    cmd: Vec<char>,
    cmd_mode: bool,
}

struct CommandParameters<'a> {
    command: &'a str,
    parameters: Vec<u16>,
}

impl TerminfoXterm256 {

    pub fn new() -> TerminfoXterm256 {
        TerminfoXterm256 {
            cmd: vec![],
            cmd_mode: false,
        }
    }

}

impl Terminfo for TerminfoXterm256 {
    
    fn parse_output(&mut self, c: u8) -> Vec<Command> {
        match self.cmd_mode {
            false => match c {
                0 => vec![],
                8 => vec![CursorLeft],
                10 => vec![CursorDown],
                13 => vec![CarriageReturn],
                27 => { self.cmd_mode = true; vec![] }
                c @ 1...255 => vec![PrintChar(c as char)],
                _ => panic!("Invalid value!"),
            },
            true => {
                self.cmd.push(c as char);
                match self.parse_command() {
                    IncompleteCommand => { 
                        vec![]
                    },
                    cmd @ _ => {
                        self.cmd_mode = false;
                        self.cmd.clear();
                        vec![cmd]
                    }
                }
            },
        }
    }

    fn parse_input(&self, key: &Key) -> Vec<u8>
    {
        match key {
            &Char(c) => vec![c as u8],
            _ => vec![],
        }
    }

}

impl TerminfoXterm256 {

    fn parse_command(&self) -> Command {
        /* TODO - use this syntax when avaliable
        match &self.cmd[..] {
            ['[', 'H', '\x1b', '[', '2', 'J'] => ClearScreen,
            _ => IncompleteCommand,
        }*/

        // Local cursor movement
        let s: String = self.cmd.iter().cloned().collect();
        match s.as_ref() {
            "[C" => CursorRight,
            "[A" => CursorUp,
            "[H" => CursorHome,
            "[2J" => ClearScreen,
            _ => {
                if self.cmd[1].is_digit(10) {
                    let parsed = self.parse_parameters();
                    match parsed.command {
                        "[_B" => CursorPDown(parsed.parameters[0]),
                        _ => IncompleteCommand,
                    }
                } else {
                    IncompleteCommand
                }
            }
        }
    }


    fn parse_parameters(&self) -> CommandParameters {
        CommandParameters { command: "", parameters: vec![] }
    }

}

// vim: ts=4:sw=4:sts=4:expandtab
