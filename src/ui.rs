extern crate ncurses;

use crate::Todo;
use ncurses::{noecho, initscr, keypad, stdscr, start_color, init_pair, COLOR_PAIR, attr_on, addstr, attr_off, refresh, getch, COLOR_GREEN, COLOR_BLACK, attr_t, COLOR_RED};
use std::char::from_digit;
// use std::char;

type Pair = i16;
static PAIR_DEFAULT: Pair = 1;
static PAIR_STUFF: Pair = 2;

const UP_ARROW: i32 = 'A' as i32;
const DOWN_ARROW: i32 = 'B' as i32;
const RIGHT_ARROW: i32 = 'C' as i32;
const LEFT_ARROW: i32 = 'D' as i32;

const ENTER:u32 = 13;
const ESC:u32 = 27;

pub fn init() {
    initscr();
    noecho();
    keypad(stdscr(), true);
    start_color();

    init_pair(PAIR_DEFAULT, COLOR_GREEN, COLOR_BLACK);
    init_pair(PAIR_STUFF, COLOR_RED, COLOR_BLACK);
}

/**
 Mutates the 'writer' string
 */
pub fn read_loop(writer: &mut String) {
    ncurses::echo();
    print_colored(&String::from("hewwo"), PAIR_DEFAULT);
    addstr("\r");
    loop {
        let ch1 = getch() as u32;

        if ch1 == ENTER || ch1 == ESC || ch1 == ncurses::KEY_LEFT as u32 {
            break;
        }
        let input_char = std::char::from_u32(ch1).unwrap();
        writer.push(input_char);
        // addstr(input_char.to_string().as_str());
    }
    print_colored(writer, PAIR_STUFF);
    noecho();
    getch();
}
// pub fn read_stuff() {
//     let color_pair: attr_t = COLOR_PAIR(PAIR_DEFAULT);
//     let pre_ch = getch();
//     attr_on(color_pair);
//     if pre_ch == ESC {
//         // ignore this one
//         getch();
//     }
//     let ch = getch();
//     match ch {
//         LEFT_ARROW => {
//             addstr("Left!");
//             ();
//         },
//         RIGHT_ARROW => {
//             addstr("Right!!");
//             ();
//         },
//         DOWN_ARROW => {
//             addstr("dddown");
//             ();
//         },
//         UP_ARROW => {
//             addstr("upppppXD");
//             ();
//         }
//         _ => {
//             addstr("wtf!!!!");
//             ();
//         },
//     }
//     refresh();
//     attr_off(color_pair);
//     getch();
//     getch();
// }

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

pub fn print_colored(input: &String, color_pair: Pair) {
    // set_bg_color(bg_color);
    let color_pair = COLOR_PAIR(color_pair);
    attr_on(color_pair);
    addstr(input.as_str());
    // print!("{}", input);
    attr_off(color_pair);
}

fn print_spaces(n: usize) {
    addstr(" ".repeat(n).as_str());
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
    print_colored(&is_done_output, PAIR_STUFF);
    print_spaces(2usize);
    print_colored(title, PAIR_STUFF);
    print_spaces(2usize);
    print_colored(desc, PAIR_STUFF);
    refresh();
}
