extern crate ncurses;

use crate::Todo;
use std::io::stdout;
use std::{io::Write};
use ncurses::{noecho, initscr, keypad, stdscr, start_color, init_pair, COLOR_PAIR, attr_on, addstr, attr_off, refresh, getch, KEY_LEFT, KEY_RIGHT, KEY_DOWN, COLOR_GREEN, COLOR_BLACK, attr_t, COLOR_RED};
// use std::char;

static PAIR_DEFAULT: i16 = 1;
static PAIR_STUFF: i16 = 2;

const UP_ARROW: i32 = 'A' as i32;
const DOWN_ARROW: i32 = 'B' as i32;
const RIGHT_ARROW: i32 = 'C' as i32;
const LEFT_ARROW: i32 = 'D' as i32;

const ENTER:i32 = 13;
const ESC:i32 = 27;

pub fn init() {
    initscr();
    noecho();
    keypad(stdscr(), false);
    start_color();

    init_pair(PAIR_DEFAULT, COLOR_GREEN, COLOR_BLACK);
    init_pair(PAIR_STUFF, COLOR_RED, COLOR_BLACK);
}

pub fn read_stuff() {
    let color_pair: attr_t = COLOR_PAIR(PAIR_DEFAULT);
    let pre_ch = getch();
    attr_on(color_pair);
    if pre_ch == ESC {
        // ignore this one
        getch();
    }
    let ch = getch();
    match ch {
        LEFT_ARROW => {
            addstr("Left!");
            ();
        },
        RIGHT_ARROW => {
            addstr("Right!!");
            ();
        },
        DOWN_ARROW => {
            addstr("dddown");
            ();
        },
        UP_ARROW => {
            addstr("upppppXD");
            ();
        }
        _ => {
            addstr("wtf!!!!");
            ();
        },
    }
    refresh();
    attr_off(color_pair);
    getch();
    getch();
}

pub fn stop() {
    getch();
    ncurses::endwin();
}

pub fn clear_screen() {
    print!("\x1B[2J");
}

pub fn new_term() {
    print!("\x1b[?1049h");
    // print!("\x1b[41muwuwuwu");
    // print!("\x1b[0m");
    // print!("hello");
}

pub fn restore_term() {
    print!("\x1b[?1049l");
}

pub enum TermBackgroundColors {
    Default,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    LightBlue,
    White
}

fn set_bg_color(bg_color: TermBackgroundColors) {
    let mut output = String::from("\x1b[");
    match bg_color {
        TermBackgroundColors::Default => output.push_str("49m"),
        TermBackgroundColors::Black => output.push_str("40m"),
        TermBackgroundColors::Red => output.push_str("41m"),
        TermBackgroundColors::Green => output.push_str("42m"),
        TermBackgroundColors::Yellow => output.push_str("43m"),
        TermBackgroundColors::Blue => output.push_str("44m"),
        TermBackgroundColors::LightBlue => output.push_str("104m"),
        TermBackgroundColors::White => output.push_str("107m")
    }
    output.push_str("\x1b[30m");
    print!("{}", output);
}

fn reset_bg_color() {
    print!("\x1b[0m");
}
pub fn print_colored(input: &String, bg_color: TermBackgroundColors) {
    // set_bg_color(bg_color);
    let color_pair = COLOR_PAIR(PAIR_STUFF);
    attr_on(color_pair);
    addstr(input as &str);
    print!("{}", input);
    attr_off(color_pair);
}

fn print_spaces(n: usize) {
    addstr(&" ".repeat(n));
}

// display a todo
pub fn print_todo(todo: &Todo) {
    let desc = &todo.description;
    let is_done = todo.is_done;
    let title = &todo.title;
    let mut is_done_output = String::from("[");
    match is_done {
        true => is_done_output.push_str("x]"),
        false => is_done_output.push_str(" ]")
    }
    print_spaces(2usize);
    print_colored(&is_done_output, TermBackgroundColors::LightBlue);
    print_spaces(2usize);
    print_colored(title, TermBackgroundColors::Red);
    print_spaces(2usize);
    print_colored(desc, TermBackgroundColors::White);
    refresh();
}
