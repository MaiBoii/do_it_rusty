use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Date {
    Todos: Vector<Todos>
}

#[derive(Serialize, Deserialize)]
pub struct Todos {
    id: u32,
    title: String,
    completed: String,
}