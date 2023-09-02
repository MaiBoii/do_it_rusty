use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Date {
    pub(crate) today: String,
    pub(crate) todos: Vec<Todo>
}

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub(crate) id: u32,
    pub(crate) content: String,
    pub(crate) completed: bool,
}