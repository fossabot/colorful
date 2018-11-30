//! Colored your terminal.
//! Usage:
//!
//!
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

use core::colors::Color;
use core::style::Style;
use core::symbols::Symbol;

pub mod core;


pub struct ColorfulString {
    text: String,
    foreground_color: Option<Color>,
    background_color: Option<Color>,
    // Support multiple style
    styles: Option<Vec<Style>>,
    is_plain: bool,
}

impl Default for ColorfulString {
    fn default() -> ColorfulString {
        ColorfulString {
            text: String::default(),
            foreground_color: None,
            background_color: None,
            styles: None,
            is_plain: true,
        }
    }
}

pub trait StrMarker {
    fn to_str(&self) -> String;
    fn get_style(&self) -> Option<Vec<Style>> { None }
    fn get_fg_color(&self) -> Option<Color> { None }
    fn get_bg_color(&self) -> Option<Color> { None }
}

impl<'a> StrMarker for &'a str {
    fn to_str(&self) -> String {
        String::from(*self)
    }
}

impl StrMarker for ColorfulString {
    fn to_str(&self) -> String {
        self.text.to_owned()
    }
    fn get_style(&self) -> Option<Vec<Style>> {
        self.styles.clone()
    }
    fn get_fg_color(&self) -> Option<Color> {
        self.foreground_color.clone()
    }
    fn get_bg_color(&self) -> Option<Color> {
        self.background_color.clone()
    }
}

pub trait Base {
    fn color(self, color: Color) -> ColorfulString;
    fn style(self, style: Style) -> ColorfulString;
}

impl<T> Base for T where T: StrMarker {
    fn color(self, color: Color) -> ColorfulString {
        ColorfulString {
            text: String::from(self.to_str()),
            foreground_color: Some(color), // color will replace
            background_color: self.get_bg_color(),
            styles: self.get_style(),
            is_plain: false,
        }
    }
    fn style(self, style: Style) -> ColorfulString {
        ColorfulString {
            text: String::from(self.to_str()),
            styles: match self.get_style() {
                Some(mut v) => {
                    v.push(style);
                    Some(v)
                }
                _ => { Some(vec![style]) }
            },
            foreground_color: self.get_fg_color(),
            background_color: self.get_bg_color(),
            is_plain: false,
        }
    }
}

pub trait ColorStyleInterface {
    // style
    fn bold(self) -> ColorfulString;
    fn blink(self) -> ColorfulString;
    fn dim(self) -> ColorfulString;
    fn underlined(self) -> ColorfulString;
    fn reverse(self) -> ColorfulString;
    fn hidden(self) -> ColorfulString;
    // color
    fn black(self) -> ColorfulString;
    fn red(self) -> ColorfulString;
    fn green(self) -> ColorfulString;
    fn yellow(self) -> ColorfulString;
    fn blue(self) -> ColorfulString;
    fn magenta(self) -> ColorfulString;
    fn cyan(self) -> ColorfulString;
    fn light_gray(self) -> ColorfulString;
    fn dark_gray(self) -> ColorfulString;
    fn light_red(self) -> ColorfulString;
    fn light_green(self) -> ColorfulString;
    fn light_yellow(self) -> ColorfulString;
    fn light_blue(self) -> ColorfulString;
    fn light_magenta(self) -> ColorfulString;
    fn light_cyan(self) -> ColorfulString;
    fn white(self) -> ColorfulString;
}


impl<T> ColorStyleInterface for T where T: Base {
    // style
    fn bold(self) -> ColorfulString { self.style(Style::Bold) }
    fn blink(self) -> ColorfulString { self.style(Style::Blink) }
    fn dim(self) -> ColorfulString { self.style(Style::Dim) }
    fn underlined(self) -> ColorfulString { self.style(Style::Underlined) }
    fn reverse(self) -> ColorfulString { self.style(Style::Reverse) }
    fn hidden(self) -> ColorfulString { self.style(Style::Hidden) }
    // color
    fn black(self) -> ColorfulString { self.color(Color::Black) }
    fn red(self) -> ColorfulString { self.color(Color::Red) }
    fn green(self) -> ColorfulString { self.color(Color::Green) }
    fn yellow(self) -> ColorfulString { self.color(Color::Yellow) }
    fn blue(self) -> ColorfulString { self.color(Color::Blue) }
    fn magenta(self) -> ColorfulString { self.color(Color::Magenta) }
    fn cyan(self) -> ColorfulString { self.color(Color::Cyan) }
    fn light_gray(self) -> ColorfulString { self.color(Color::LightGray) }
    fn dark_gray(self) -> ColorfulString { self.color(Color::DarkGray) }
    fn light_red(self) -> ColorfulString { self.color(Color::LightRed) }
    fn light_green(self) -> ColorfulString { self.color(Color::LightGreen) }
    fn light_yellow(self) -> ColorfulString { self.color(Color::LightYellow) }
    fn light_blue(self) -> ColorfulString { self.color(Color::LightBlue) }
    fn light_magenta(self) -> ColorfulString { self.color(Color::LightMagenta) }
    fn light_cyan(self) -> ColorfulString { self.color(Color::LightCyan) }
    fn white(self) -> ColorfulString { self.color(Color::White) }
}

pub trait ExtraColorInterface {
    fn grey0(self) -> ColorfulString;
}

impl<T> ExtraColorInterface for T where T: Base {
    fn grey0(self) -> ColorfulString { self.color(Color::Grey0) }
}


impl Display for ColorfulString {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let mut is_init = false;
        if self.is_plain {
            write!(f, "{}", self.text)?;
            Ok(())
        } else {
            match &self.foreground_color {
                Some(v) => {
                    is_init = true;
                    f.write_str(Symbol::Screen256Foreground.to_str())?;
                    f.write_str(v.to_str())?;
                }
                _ => {}
            }
            match &self.background_color {
                Some(v) => {
                    is_init = true;
                    f.write_str(Symbol::Screen256Background.to_str())?;
                    f.write_str(v.to_str())?;
                }
                _ => {}
            }

            match &self.styles {
                Some(v) => {
                    if !is_init {
                        write!(f, "{}{}", Symbol::Esc, Symbol::LeftBrackets)?;
                    } else {
                        f.write_str(Symbol::Semicolon.to_str())?;
                    }
                    let t: Vec<String> = v.into_iter().map(|x| x.to_string()).collect();
                    f.write_str(&t.join(";")[..])?;
                }
                _ => {}
            }
            f.write_str(Symbol::Mode.to_str())?;
            write!(f, "{}", self.text)?;
            f.write_str(Symbol::Reset.to_str())?;
            Ok(())
        }
    }
}