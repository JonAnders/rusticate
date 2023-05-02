#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TodoItem {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub completed: bool,
}
