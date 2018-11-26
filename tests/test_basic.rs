extern crate colorful;
extern crate core;

use colorful::Colorful;
use colorful::core::Color;

#[test]
fn test_1() {
    assert_eq!("\u{1b}", "\x1B");
}

#[test]
fn test_color() {
    let s = "Hello world";
    assert_eq!("\x1B[38;5;1mHello world\x1B[0m".to_owned(), s.color(Color::Red).to_string());
    assert_eq!("\x1B[38;5;220mHello world\x1B[0m".to_owned(), s.color(Color::Red).color(Color::Gold1).to_string());
}


#[test]
fn test_style() {
    let s = "Hello world";
    assert_eq!("\x1B[1mHello world\x1B[0m".to_owned(), s.bold().to_string());
    assert_eq!("\x1B[1;5mHello world\x1B[0m".to_owned(), s.bold().blink().to_string());
}

#[test]
fn test_mix() {
    let s = "Hello world";
    assert_eq!("\x1B[38;5;1;5mHello world\x1B[0m".to_owned(), s.color(Color::Red).blink().to_string());
    assert_eq!("\x1B[38;5;220;1mHello world\x1B[0m".to_owned(), s.bold().color(Color::Gold1).to_string());

    assert_eq!("\x1B[38;5;2;5;1mHello world\x1B[0m".to_owned(), s.color(Color::Green).blink().bold().to_string());
    assert_eq!("\x1B[38;5;220;1;5mHello world\x1B[0m".to_owned(), s.bold().blink().color(Color::Gold1).to_string());
}