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
    pars: Vec<u16>,
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
        let s: String = self.cmd.iter().cloned().collect();
        match s.as_ref() {
            // local cursor movement
            "[C"  => CursorRight,
            "[A"  => CursorUp,
            "[H"  => CursorHome,
            "7"   => SaveCursorPosition,
            "8"   => RestoreCursorPosition,
            // scrolling
            "M"   => CursorUp,  // TODO - ???
            // add to screen
            "[L"  => InsertLine,
            // delete from screen
            "[2J" => ClearScreen,
            "[P"  => DeleteChar,
            "[M"  => DeleteLine,
            "[J"  => ClearEOS,
            "[K"  => ClearEOL,
            "[1K" => ClearBOL,
            // insert mode
            "[4h" => SetInsertMode(true),
            "[4l" => SetInsertMode(false),
            // attributes
            "[7m" => SetStandoutMode(true),
            "[27m" => SetStandoutMode(false),
            "[4m" => SetUnderlineMode(true),
            "[24m" => SetUnderlineMode(false),
            _ => {
                if self.cmd.len() > 1 && self.cmd[1].is_digit(10) {
                    let p = self.parse_parameters();
                    match p.command.as_ref() {
                        // parameterized local cursor movement
                        "[_D" => CursorPLeft(p.pars[0]),
                        "[_B" => CursorPDown(p.pars[0]),
                        "[_C" => CursorPRight(p.pars[0]),
                        "[_A" => CursorPUp(p.pars[0]),
                        // absolute cursor movement
                        "[_;_H" => MoveCursor(p.pars[0], p.pars[1]),
                        "[_G"   => MoveCursorColumn(p.pars[0]),
                        "[_d"   => MoveCursorRow(p.pars[0]),
                        // scrolling
                        "[_;_r" => ChangeScrollRegion(p.pars[0], p.pars[1]),
                        "[_S"   => ScrollForward(p.pars[0]),
                        "[_T"   => ScrollReverse(p.pars[0]),
                        // add to screen
                        "[_L" => InsertLines(p.pars[0]),
                        // delete from screen
                        "[_P" => DeleteChars(p.pars[0]),
                        "[_M" => DeleteLines(p.pars[0]),
                        "[_X" => EraseChars(p.pars[0]),
                        // insert mode
                        "[_@" => InsertChars(p.pars[0]),
                        // no real code
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
        CommandParameters { command: cmd.iter().cloned().collect(), pars: pars }
    }

}

// vim: ts=4:sw=4:sts=4:expandtab
