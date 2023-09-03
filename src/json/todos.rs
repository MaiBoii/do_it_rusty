use serde::{Deserialize, Serialize};

//날짜 단위
#[derive(Serialize, Deserialize)]
pub struct Date {
    pub(crate) today: String,
    pub(crate) todos: Vec<Todo>
}


//일정 단위
#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub(crate) id: usize,
    pub(crate) content: String,
    pub(crate) completed: bool,
}