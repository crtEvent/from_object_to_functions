use serde::Deserialize;

#[derive(Deserialize)]
pub struct AddItemRequest {
    pub item_name: String,
    pub due_date: String,
    pub status: String,
}
