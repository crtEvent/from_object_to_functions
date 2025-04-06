use crate::zettai::domain::{ToDoList, User};
use crate::zettai::page::end_page::end_page;
use crate::zettai::page::show_list_page::list_page;
use axum::routing::get;
use axum::Router;
use axum::extract::Path;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Zettai {
    lists: Arc<Mutex<HashMap<User, Vec<ToDoList>>>>,
}

impl Zettai {
    pub fn new(lists: HashMap<User, Vec<ToDoList>>) -> Self {
        Zettai { lists: Arc::new(Mutex::new(lists)), }
    }

    pub async fn serve(self, port: u16) {

        let lists = self.lists.clone();

        let app = Router::new()
            .route("/", get(end_page()))
            .route("/todo/{user_name}/{list_name}",
                   get(move |path: Path<(String, String)>| {
                    let lists = lists.clone();
                    async move { list_page(&lists, path) }
                }));

        let addr = format!("127.0.0.1:{}", port);
        let listener = tokio::net::TcpListener::bind(addr.clone()).await.unwrap();
        println!("Server started at http://{}", addr);
        axum::serve(listener, app).await.unwrap();
    }
}
