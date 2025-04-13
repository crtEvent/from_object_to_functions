use std::sync::{Arc, Mutex};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::Form;
use axum::response::{Html, IntoResponse, Redirect, Response};
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

    match hub.lock().unwrap()
        .add_item_to_list(&user, &list_name, &item) {
        Ok(_) => Redirect::to(&format!("/todo/{}/{}", user.name, list_name.name)).into_response(),
        Err(err_message) => (
            StatusCode::NOT_FOUND,
            Html(format!("<h1>{}</h1><p>Please try again.</p>", err_message)),
        ).into_response()
    }
}