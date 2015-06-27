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
                7 => vec![Bell],
                8 => vec![CursorLeft],
                10 => vec![CursorDown],
                13 => vec![CarriageReturn],
                27 => { self.cmd_mode = true; vec![] }
                c @ 1...255 => vec![PrintChar(c as char)],
                _ => panic!("Invalid value!"),
            },
            true => {
                self.cmd.push(c as char);
                let cmds = self.parse_command();
                if !cmds.is_empty() {
                    self.cmd_mode = false;
                    self.cmd.clear();
                }
                cmds
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

    fn parse_command(&self) -> Vec<Command> {
        let s: String = self.cmd.iter().cloned().collect();
        match s.as_ref() {
            // local cursor movement
            "[C"  => vec![CursorRight],
            "[A"  => vec![CursorUp],
            "[H"  => vec![CursorHome],
            "7"   => vec![SaveCursorPosition],
            "8"   => vec![RestoreCursorPosition],
            // scrolling
            "M"   => vec![CursorUp],  // TODO - ???
            // add to screen
            "[L"  => vec![InsertLine],
            // delete from screen
            "[2J" => vec![ClearScreen],
            "[P"  => vec![DeleteChar],
            "[M"  => vec![DeleteLine],
            "[J"  => vec![ClearEOS],
            "[K"  => vec![ClearEOL],
            "[1K" => vec![ClearBOL],
            // insert mode
            "[4h" => vec![SetInsertMode(true)],
            "[4l" => vec![SetInsertMode(false)],
            // attributes
            "[27m" => vec![SetStandoutMode(false)],
            "[24m" => vec![SetUnderlineMode(false)],
            "[m"   => vec![ExitAttributeMode],
            "(0"   => vec![SetCharsetMode(true)],
            "(B"   => vec![SetCharsetMode(false)],
            // bells
            "[?5h" => vec![ReverseScreen(true)],
            "[?5l" => vec![ReverseScreen(false)],
            // cursor intensity
            "[?25l"          => vec![CursorVisibility(0)],
            "[?12l\x1b[?25h" => vec![CursorVisibility(1)],
            "[?12;25h"       => vec![CursorVisibility(2)],
            // meta key
            "[?1034h" => vec![SetMetaMode(true)],
            "[?1034l" => vec![SetMetaMode(false)],
            // program initialization
            "[?1049h" => vec![SaveScreen],
            "[?1049l" => vec![RestoreScreen],
            // keypad keys activation
            "[?1h\x1b=" => vec![SetKeypadMode(true)],
            "[?1l\x1b>" => vec![SetKeypadMode(false)],
            // local printer
            "[i"  => vec![NoOp],
            "[4i" => vec![NoOp],
            "[5i" => vec![NoOp],
            _ => {
                if self.cmd.len() > 1 && self.cmd[1].is_digit(10) {
                    let p = self.parse_parameters();
                    match p.command.as_ref() {
                        // parameterized local cursor movement
                        "[_D" => vec![CursorPLeft(p.pars[0])],
                        "[_B" => vec![CursorPDown(p.pars[0])],
                        "[_C" => vec![CursorPRight(p.pars[0])],
                        "[_A" => vec![CursorPUp(p.pars[0])],
                        // absolute cursor movement
                        "[_;_H" => vec![MoveCursor(p.pars[0], p.pars[1])],
                        "[_G"   => vec![MoveCursorColumn(p.pars[0])],
                        "[_d"   => vec![MoveCursorRow(p.pars[0])],
                        // scrolling
                        "[_;_r" => vec![ChangeScrollRegion(p.pars[0], p.pars[1])],
                        "[_S"   => vec![ScrollForward(p.pars[0])],
                        "[_T"   => vec![ScrollReverse(p.pars[0])],
                        // add to screen
                        "[_L" => vec![InsertLines(p.pars[0])],
                        // delete from screen
                        "[_P" => vec![DeleteChars(p.pars[0])],
                        "[_M" => vec![DeleteLines(p.pars[0])],
                        "[_X" => vec![EraseChars(p.pars[0])],
                        // insert mode
                        "[_@" => vec![InsertChars(p.pars[0])],
                        // attributes
                        "[_m"                 => self.sgr_attributes(p.pars),
                        "[_;_m"               => self.sgr_attributes(p.pars),
                        "[_;_;_m"             => self.sgr_attributes(p.pars),
                        "[_;_;_;_m"           => self.sgr_attributes(p.pars),
                        "[_;_;_;_;_m"         => self.sgr_attributes(p.pars),
                        "[_;_;_;_;_;_m"       => self.sgr_attributes(p.pars),
                        "[_;_;_;_;_;_;_m"     => self.sgr_attributes(p.pars),
                        "[_;_;_;_;_;_;_;_m"   => self.sgr_attributes(p.pars),
                        "[_;_;_;_;_;_;_;_;_m" => self.sgr_attributes(p.pars),
                        // no real code
                        _ => vec![],
                    }
                } else {
                    vec![]
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


    fn sgr_attributes(&self, pars: Vec<u16>) -> Vec<Command> {
        let mut cmds: Vec<Command> = Vec::new();
        for p in pars {
            cmds.push(match p {
                0 => ExitAttributeMode,
                1 => SetBoldMode,
                4 => SetUnderlineMode(true),
                5 => SetBlinkMode,
                7 => SetStandoutMode(true),
                8 => SetInvisibleMode,
                _ => NoOp,
            });
        }
        cmds
    }

}

// vim: ts=4:sw=4:sts=4:expandtab
