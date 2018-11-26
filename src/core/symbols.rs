use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

pub enum Symbol {
    Mode,
    Semicolon,
    Tmp,
    Esc,
    Reset,
    Screen256Foreground,
    Screen256Background,
}

impl Symbol {
    pub fn to_str<'a>(&self) -> &'a str {
        match self {
            Symbol::Mode => "m",
            Symbol::Semicolon => ";",
            Symbol::Tmp => "[",
            Symbol::Esc => "\x1B",
            Symbol::Reset => "\x1B[0m",
            Symbol::Screen256Foreground => "\x1B[38;5;",
            Symbol::Screen256Background => "\x1B[48;5;",
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.to_str())
    }
}