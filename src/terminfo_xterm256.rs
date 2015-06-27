use terminfo::Terminfo;
use command::Command;
use command::Command::*;
use userevent::Key;
use userevent::Key::*;

pub struct TerminfoXterm256 {
    cmd: Vec<char>,
    cmd_mode: bool,
}

struct CommandParameters<> {
    command: String,
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
        // Local cursor movement
        let s: String = self.cmd.iter().cloned().collect();
        match s.as_ref() {
            "[C" => CursorRight,
            "[A" => CursorUp,
            "[H" => CursorHome,
            "[2J" => ClearScreen,
            _ => {
                if self.cmd.len() > 1 && self.cmd[1].is_digit(10) {
                    let parsed = self.parse_parameters();
                    match parsed.command.as_ref() {
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
        let mut par = 0u16;
        let mut pars: Vec<u16> = Vec::new();
        let mut cmd: Vec<char> = Vec::new();
        let mut parsing_digit = false;
        for c in &self.cmd {
            if c.is_digit(10) {
                parsing_digit = true;
                par = par*10 + (*c as u8 - '0' as u8) as u16;
                if *cmd.last().unwrap() != '_' { 
                    cmd.push('_'); 
                }
            } else {
                cmd.push(*c);
                if parsing_digit {
                    pars.push(par);
                    par = 0;
                    parsing_digit = false;
                }
            }
        }
        CommandParameters { command: cmd.iter().cloned().collect(), parameters: pars }
    }

}

// vim: ts=4:sw=4:sts=4:expandtab
