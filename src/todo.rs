use std::fmt;

pub struct Todo{
    pub is_done: bool,
    pub title: String,
    pub description: String
}

impl Todo{
    pub fn toggle_todo(&mut self) {
        self.is_done = !self.is_done;
    }

    pub fn set_title(&mut self, new_title: String) {
        self.title = new_title;
    }

    pub fn set_description(&mut self, new_description: String) {
        self.description = new_description;
    }
}

impl fmt::Debug for Todo{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Todo")
        .field("isdone", &self.is_done)
        .field("title", &self.title)
        .field("descr", &self.description)
        .finish()
    }
}