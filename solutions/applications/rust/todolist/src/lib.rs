use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct User {
    #[serde(with = "uuid::serde::bytes")]
    person_id: Uuid,
    person_name: String,
    todolist: Vec<TodoList>,
}

#[derive(Serialize, Deserialize)]
struct TodoList {
    id: u32,
    title: String,
    descriptions: String,
}
