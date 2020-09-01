use std::fmt;
use crate::todo::Todo;

pub struct List{
    pub todos: Vec<Todo>
}

impl List{
    pub fn new() -> List{
        List{
            todos: Vec::new(),
        }
    }

    pub fn add_todo_raw(&mut self, title: String, description: String) {
        let todo = Todo{
            is_done: false,
            title,
            description
        };
        self.todos.push(todo);
    }

    pub fn add_todo(&mut self, todo: Todo) {
        self.todos.push(todo);
    }
}

impl Default for List {
    fn default() -> Self {
        List::new()
    }
}

impl fmt::Debug for List{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("List")
        .field("todos", &self.todos)
        .finish()
    }
}
