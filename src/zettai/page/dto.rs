use serde::Deserialize;

#[derive(Deserialize)]
pub struct AddItemRequest {
    pub item_name: String,
}