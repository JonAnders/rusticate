use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoItem {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub completed: bool,
}
