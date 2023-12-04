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

    pub fn toggle(&mut self, idx: usize) {
        let todo = self.todos.iter_mut().nth(idx).unwrap();
        todo.completed = !todo.completed;
    }
}

#[derive(Debug)]
pub struct ToDo {
    pub text: String,
    pub completed: bool,
}

impl ToDo {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            completed: false,
        }
    }
}
