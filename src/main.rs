extern crate todo_app;

use todo_app::List;
use todo_app::Db;

fn main() {
    println!("Hello, world!");
    let list = &mut List::new();
    list.add_todo_raw("Clean up toilet".to_string(), "Sanitize with poopoo".to_string());
    list.add_todo_raw("boingo".to_string(), "woingo".to_string());
    let db = &mut Db::new();
    db.make_new_table().unwrap();
    let list_id = db.set_list(list).unwrap();
    let output = db.get_list(list_id);
    dbg!(output);
    
}

// Overview: Has list of todos
// List: Todos, Name
// Todo: Checkbox, Title, Description, 

