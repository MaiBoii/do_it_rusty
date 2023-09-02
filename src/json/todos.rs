use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Todo {
    id: u32,
    content: String,
    completed: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Date {
    today: String,
    todos: Vector<Todo>
}