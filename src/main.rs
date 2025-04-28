mod exercises;
mod zettai;

use crate::zettai::business::domain::{ToDoList, User};
use crate::zettai::business::todolist_fetcher::ToDoListFetcherFromMap;
use crate::zettai::business::zettai_hub::ToDoListHub;
use crate::zettai::zettai::Zettai;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let ape = User {
        name: "ape".to_string(),
    };
    let ape_list = ToDoList::new("book", vec!["write chapter", "insert code", "draw diagram"]);

    let lists = HashMap::from([(
        ape,
        HashMap::from([(ape_list.list_name.clone(), ape_list.clone())]),
    )]);

    let fetcher = ToDoListFetcherFromMap::new(lists);

    let hub = ToDoListHub::new(fetcher);

    let app = Zettai::new(Arc::new(Mutex::new(hub)));
    println!("Server started at http://localhost:8080/todo/ape/book");
    app.serve(8080).await;
}
