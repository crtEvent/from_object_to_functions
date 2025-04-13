use std::sync::{Arc, Mutex};
use crate::zettai::page::end_page::end_page;
use crate::zettai::page::show_list_page::list_page;
use axum::routing::{get, post};
use axum::{Form, Router};
use axum::extract::Path;
use crate::zettai::business::zettai_hub::ZettaiHub;
use crate::zettai::page::add_new_item::add_new_item;
use crate::zettai::page::dto::AddItemRequest;

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
            .route("/todo/{user_name}/{list_name}",
                   get({
                       let hub = self.hub.clone();
                       move |path: Path<(String, String)>| {
                       async move { list_page(hub, path) }
                   }}))
            .route("/todo/{user_name}/{list_name}",
                   post({
                       let hub = self.hub.clone();
                       move |path: Path<(String, String)>, form: Form<AddItemRequest>| {
                           async move { add_new_item(hub, path, form) }
                   }}));

        let addr = format!("127.0.0.1:{}", port);
        let listener = tokio::net::TcpListener::bind(addr.clone()).await.unwrap();
        println!("Server started at http://{}", addr);
        axum::serve(listener, router).await.unwrap();
    }
}
