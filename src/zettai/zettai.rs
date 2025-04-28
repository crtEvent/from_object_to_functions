use crate::zettai::business::zettai_hub::ZettaiHub;
use crate::zettai::response::add_new_item::add_new_item;
use crate::zettai::response::dto::AddItemRequest;
use crate::zettai::response::get_end_page::end_page;
use crate::zettai::response::get_item_list_page::get_item_list_page;
use axum::extract::Path;
use axum::routing::{get, post};
use axum::{Form, Router};
use std::sync::{Arc, Mutex};

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
                    move |path: Path<(String, String)>| async move { get_item_list_page(hub, path) }
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
            );

        let addr = format!("127.0.0.1:{}", port);
        let listener = tokio::net::TcpListener::bind(addr.clone()).await.unwrap();
        println!("Server started at http://{}", addr);
        axum::serve(listener, router).await.unwrap();
    }
}
