use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ToDoItem {
    pub title: String,
    pub status: String
}