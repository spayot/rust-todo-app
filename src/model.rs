#[derive(Debug)]
pub struct Model {
    pub todos: Vec<ToDo>,
    pub inputs: String,
}

impl Model {
    pub fn new() -> Model {
        Model {
            todos: vec![],
            inputs: String::new(),
        }
    }
    pub fn add(&mut self, text: &str) {
        self.todos.push(ToDo::new(text))
    }
    pub fn remove(&mut self, idx: usize) {
        self.todos.remove(idx);
    }

    pub fn remove_all(&mut self) {
        self.todos = vec![];
    }

    pub fn total(&self) -> usize {
        self.todos.len()
    }
}

#[derive(Debug)]
struct ToDo {
    text: String,
    completed: bool,
}

impl ToDo {
    fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            completed: false,
        }
    }

    pub fn completed(&self) -> bool {
        self.completed
    }
}
