use crate::zettai::business::zettai_hub::ZettaiHub;
use crate::zettai::response::add_new_item::add_new_item;
use crate::zettai::response::dto::{AddItemRequest, CreateTodoListRequest};
use crate::zettai::response::get_all_todo_lists::get_all_todo_lists;
use crate::zettai::response::get_end_page::end_page;
use crate::zettai::response::get_todo_list::get_todo_list;
use axum::extract::Path;
use axum::routing::{get, post};
use axum::{Form, Router};
use std::sync::{Arc, Mutex};
use crate::zettai::response::create_new_todo_list::create_new_todo_list;

pub struct Zettai {
    hub: Arc<Mutex<dyn ZettaiHub>>,
}

impl Zettai {
    pub fn new(hub: Arc<Mutex<dyn ZettaiHub>>) -> Zettai {
        Zettai { hub }
    }

    pub async fn serve(self, port: u16) {
        let router = Router::new()
            .route("/", get(end_page()))
            .route(
                "/todo/{user_name}/{list_name}",
                get({
                    let hub = self.hub.clone();
                    move |path: Path<(String, String)>| async move { get_todo_list(hub, path) }
                }),
            )
            .route(
                "/todo/{user_name}/{list_name}",
                post({
                    let hub = self.hub.clone();
                    move |path: Path<(String, String)>, form: Form<AddItemRequest>| async move {
                        add_new_item(hub, path, form)
                    }
                }),
            )
            .route(
                "/todo/{user_name}",
                get({
                    let hub = self.hub.clone();
                    move |path: Path<String>| async move { get_all_todo_lists(hub, path) }
                }),
            )
            .route(
                "/todo/{user_name}",
                post({
                    let hub = self.hub.clone();
                    move |path: Path<String>, form: Form<CreateTodoListRequest>| 
                        async move { create_new_todo_list(hub, path, form) }
                }),
            );

        let addr = format!("127.0.0.1:{}", port);
        let listener = tokio::net::TcpListener::bind(addr.clone()).await.unwrap();
        println!("Server started at http://{}", addr);
        axum::serve(listener, router).await.unwrap();
    }
}
