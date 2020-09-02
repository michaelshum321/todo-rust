extern crate todo_app;

use todo_app::List;
use todo_app::Db;
use todo_app::ui;
use std::{thread, time};

fn main() {
    ui::init();
    // ui::new_term();
    // println!("Hello, world!");
    let list = &mut List::new();
    list.add_todo_raw("Clean up toilet".to_string(), "Sanitize with Lysol".to_string());
    list.add_todo_raw("boingo".to_string(), "woingo".to_string());
    let db = &mut Db::new();
    db.make_new_table().unwrap();
    let list_id = db.set_list(list).unwrap();
    // println!("list id");
    // dbg!(list_id);
    let output = db.get_list(list_id);
    let a_todo = output.todos.get(0).unwrap();
    ui::print_todo(a_todo);
    // dbg!(output);
    // let dur = time::Duration::from_millis(5000);
    // ui::init();
    ui::read_stuff();
    // thread::sleep(dur);
    ui::stop();
    // ui::restore_term();
}

// Overview: Has list of todos
// List: Todos, Name
// Todo: Checkbox, Title, Description, 