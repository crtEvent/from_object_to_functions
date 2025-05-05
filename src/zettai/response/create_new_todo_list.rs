use crate::zettai::business::domain::{ListName, User};
use crate::zettai::business::domain_error::DomainError;
use crate::zettai::business::zettai_hub::ZettaiHub;
use crate::zettai::response::dto::CreateTodoListRequest;
use crate::zettai::response::get_error_page::get_error_page;
use axum::extract::Path;
use axum::response::{IntoResponse, Redirect, Response};
use axum::Form;
use std::sync::{Arc, Mutex};

pub fn create_new_todo_list(
    hub: Arc<Mutex<dyn ZettaiHub>>,
    Path(user_name): Path<String>,
    Form(dto): Form<CreateTodoListRequest>,
) -> Response {
    let user = User {
        name: user_name.to_string(),
    };
    let list_name = ListName {
        name: dto.list_name,
    };

    let result = hub.lock().unwrap().create_new_todo_list(&user, &list_name);
    match result {
        Ok(_) => Redirect::to(&format!("/todo/{}", user.name)).into_response(),
        Err(e) => match e {
            DomainError::UserNotFound => get_error_page("User not found."),
            DomainError::DuplicateTodoListName => {
                get_error_page("List name already exists. Please choose another one.")
            }
        },
    }
}
