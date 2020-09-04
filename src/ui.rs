extern crate ncurses;

use crate::Todo;
use ncurses::{noecho, initscr, keypad, stdscr, start_color, init_pair, COLOR_PAIR, attr_on, addstr, attr_off, refresh, getch, COLOR_GREEN, COLOR_BLACK, COLOR_RED, erase};
// use std::char;

type Pair = i16;
static PAIR_DEFAULT: Pair = 1;
static PAIR_STUFF: Pair = 2;
static PAIR_FOCUS_DISPLAY: Pair = 3;
static PAIR_FOCUS_INPUT: Pair = 4;

// const UP_ARROW: i32 = 'A' as i32;
// const DOWN_ARROW: i32 = 'B' as i32;
// const RIGHT_ARROW: i32 = 'C' as i32;
// const LEFT_ARROW: i32 = 'D' as i32;

const ENTER:u32 = 13;
const ESC:u32 = 27;
// const TAB:i32 = 9;

pub fn init() {
    initscr();
    noecho();
    keypad(stdscr(), true);
    start_color();

    init_pair(PAIR_DEFAULT, COLOR_GREEN, COLOR_BLACK);
    init_pair(PAIR_STUFF, COLOR_RED, COLOR_BLACK);
    init_pair(PAIR_FOCUS_DISPLAY, ncurses::COLOR_WHITE, ncurses::COLOR_BLUE);
    init_pair(PAIR_FOCUS_INPUT, ncurses::COLOR_WHITE, ncurses::COLOR_BLACK);
}

pub fn clear() {
    erase();
}

pub fn edit(content: Vec<String>, init_focus:usize) -> Vec<String>{

    let mut output: Vec<String> = Vec::new();
    for _i in 0..content.len() {
        output.push(String::new());
    }
    let mut focus = init_focus;
    loop {
        clear();
        // display headers
        attr_on(ncurses::A_UNDERLINE());
        for (i, item) in content.iter().enumerate() {

            if i == focus {
                ncurses::attr_off(COLOR_PAIR(PAIR_DEFAULT));
                attr_on(COLOR_PAIR(PAIR_FOCUS_DISPLAY));
            }else{
                attr_on(COLOR_PAIR(PAIR_DEFAULT));
            }

            // print header title
            ncurses::mv(i as i32 * 3, 0);
            addstr(item.as_str());
            // print ===== below header
            ncurses::mv((i as i32 * 3) + 1, 0);
            addstr("=====================");
            // print content 
            if i != focus {
                ncurses::mv(i as i32 * 3 + 2, 0);
                addstr(output[i].as_str());
            }


            if i == focus {
                ncurses::attr_off(COLOR_PAIR(PAIR_FOCUS_DISPLAY));
                attr_on(COLOR_PAIR(PAIR_DEFAULT));
            }
        }
        attr_off(ncurses::A_UNDERLINE());

        // display content 
        // attr_on(ncurses::A_BOLD());
        let mut row = output.get(focus).unwrap().clone();
        // attr_on(ncurses::A_BLINK());
        // ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_VERY_VISIBLE);
        // ncurses::echo();
        noecho();
        attr_on(COLOR_PAIR(PAIR_FOCUS_INPUT));
        let mut do_exit = false;    
        loop{
            let y = (focus as i32) * 3 + 2;
            ncurses::mv(y, 0);
            ncurses::clrtoeol();
            addstr(row.as_str());
            ncurses::mv((focus as i32) * 3 + 2, row.len() as i32);

            let ch1 = getch();
            // Enter does not work
            // nor does cursor goto new line?

            if ch1 == 9 { // Tab has been entered
                // TODO change focus to next boi
                // addstr("!!BAI!!!");
                output[focus] = row.to_string();
                focus = (focus + 1) % 3;
                if row.len() != 0 {

                }
                break;
            }else if ch1 == 127 {
                // addstr("back!!");
                row.pop();
                continue;
            }else if ch1 == 27 {
                do_exit = true;
                output[focus] = row.to_string();
                break;
            }

            row.push(std::char::from_u32(ch1 as u32).unwrap());
        }
        // output[focus] = row.to_string();
        
        if do_exit == true {
            ncurses::attr_off(COLOR_PAIR(PAIR_FOCUS_INPUT));
            break;
        }
    }

    output
}

// pub fn edit(content: Vec<String>, focus:usize) -> Vec<String>{
//     clear();

//     // display Titles
//     let mut output: Vec<String> = Vec::new();
    
//     attr_on(ncurses::A_BOLD());
//     for (i, item) in content.iter().enumerate() {
//         if i == focus {
//             ncurses::attr_off(COLOR_PAIR(PAIR_DEFAULT));
//             attr_on(COLOR_PAIR(PAIR_FOCUS_DISPLAY));
//         }
//         addstr(item.as_str());
//         addstr("\n");
//         addstr("=========\n\n");

//         if i == focus {
//             ncurses::attr_off(COLOR_PAIR(PAIR_FOCUS_DISPLAY));
//             attr_on(COLOR_PAIR(PAIR_DEFAULT));
//         }
//     }
//     attr_on(ncurses::A_BOLD());
//     let row = &mut String::new();
//     // attr_on(ncurses::A_BLINK());
//     // ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_VERY_VISIBLE);
//     // ncurses::echo();
//     noecho();
//     loop{
//         let y = (focus as i32) * 3 + 2;
//         ncurses::mv(y, 0);
//         ncurses::clrtoeol();
//         addstr(row.as_str());
//         ncurses::mv((focus as i32) * 3 + 2, row.len() as i32);

//         let ch1 = getch();
//         // Enter does not work
//         // nor does  cursor goto new line?

//         if ch1 == 10 {
//             // TODO change focus to next boi
//             // addstr("!!BAI!!!");
//             break;
//         }else if ch1 == 127 {
//             // addstr("back!!");
//             row.pop();
//             continue;
//         }

//         row.push(std::char::from_u32(ch1 as u32).unwrap());
//     }

//     for x in 0..content.len() {
//         if focus == x {
//             output.push(row.to_string());
//         } else {
//             output.push(String::new());
//         }
//     }

//     noecho();
//     output
// }

pub fn get_input_void() {
    getch();
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
