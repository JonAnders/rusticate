use diesel::Insertable;
use diesel::Queryable;
use crate::schema::todo_items;

// Represents a to-do list item with its details
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Insertable, Queryable, PartialEq)]
#[diesel(table_name = todo_items)]
pub struct TodoItem {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub completed: bool,
}
