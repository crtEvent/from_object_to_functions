use std::sync::{Arc, Mutex};
use axum::extract::Path;
use axum::Form;
use axum::response::{IntoResponse, Redirect, Response};
use chrono::{Local, NaiveDate};
use crate::zettai::business::domain::{ListName, ToDoItem, ToDoStatus, User};
use crate::zettai::business::zettai_hub::ZettaiHub;
use crate::zettai::page::dto::AddItemRequest;

pub fn add_new_item(
    hub: Arc<Mutex<dyn ZettaiHub>>,
    Path((user_name, list_name)): Path<(String, String)>,
    Form(dto): Form<AddItemRequest>,
) -> Response {
    let user = User { name: user_name.to_string() };
    let list_name = ListName { name: list_name.to_string() };
    let due_date = NaiveDate::parse_from_str(&*dto.due_date, "%Y-%m-%d")
        .unwrap_or_else(|_| Local::now().date_naive());

    let item = ToDoItem {
        description: dto.item_name,
        due_date,
        state: ToDoStatus::from_str(&*dto.status),
    };

    hub.lock().unwrap().add_item_to_list(&user, &list_name, &item);

    Redirect::to(&format!("/todo/{}/{}", user.name, list_name.name)).into_response()
}