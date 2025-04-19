use std::sync::{Arc, Mutex};
use axum::extract::Path;
use axum::Form;
use axum::response::{IntoResponse, Redirect, Response};
use crate::zettai::business::domain::{ListName, ToDoItem, User};
use crate::zettai::business::zettai_hub::ZettaiHub;
use crate::zettai::page::dto::AddItemRequest;

pub fn add_new_item(
    hub: Arc<Mutex<dyn ZettaiHub>>,
    Path((user_name, list_name)): Path<(String, String)>,
    Form(dto): Form<AddItemRequest>,
) -> Response {
    let user = User { name: user_name.to_string() };
    let list_name = ListName { name: list_name.to_string() };
    let item = ToDoItem { description: dto.item_name };

    hub.lock().unwrap().add_item_to_list(&user, &list_name, &item);

    Redirect::to(&format!("/todo/{}/{}", user.name, list_name.name)).into_response()
}