use std::sync::{Arc, Mutex};
use crate::zettai::page::end_page::end_page;
use crate::zettai::page::show_list_page::list_page;
use axum::routing::get;
use axum::Router;
use axum::extract::Path;
use crate::zettai::business::zettai_hub::ZettaiHub;

pub struct Zettai {
    hub: Arc<Mutex<dyn ZettaiHub>>,
}

impl Zettai {
    pub fn new(hub: Arc<Mutex<dyn ZettaiHub>>) -> Zettai {
        Zettai { hub, }
    }

    pub async fn serve(self, port: u16) {
        let app = Router::new()
            .route("/", get(end_page()))
            .route("/todo/{user_name}/{list_name}",
                   get(move |path: Path<(String, String)>| {
                    async move { list_page(self.hub, path) }
                }));

        let addr = format!("127.0.0.1:{}", port);
        let listener = tokio::net::TcpListener::bind(addr.clone()).await.unwrap();
        println!("Server started at http://{}", addr);
        axum::serve(listener, app).await.unwrap();
    }
}
