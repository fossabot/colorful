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
    fn bold(self) -> ColorfulString;
    fn blink(self) -> ColorfulString;
}


impl<T> ColorStyleInterface for T where T: Base {
    fn bold(self) -> ColorfulString {
        self.style(Style::Bold)
    }

    fn blink(self) -> ColorfulString {
        self.style(Style::Blink)
    }
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