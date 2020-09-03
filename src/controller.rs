use crate::ui;
use crate::Todo;
enum Context {
}
pub struct Controller{
    todo: Todo
}

impl Controller {
    pub fn new() -> Controller{
        Controller {
            todo: Todo::new()
        }
    }

    pub fn init(&self) {
        ui::init();
    }

    pub fn do_stuff(&self) -> Todo {
        let content: Vec<String> = vec![String::from("title"), String::from("description"), String::from("is_done")];
        let output = ui::edit(content, 1);
        let mut todo = Todo::new();
        todo.set_title(output.get(0).unwrap().to_string());
        todo.set_description(output.get(1).unwrap().to_string());
        todo
    }

    pub fn spin(&self) {
        ui::get_input_void();
    }
    pub fn stop(&self) {
        ui::stop();
    }
}