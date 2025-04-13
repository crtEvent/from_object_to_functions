mod exercises;
mod zettai;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::zettai::zettai::Zettai;
use crate::zettai::business::domain::{ToDoList, User};
use crate::zettai::business::zettai_hub::ToDoListHub;

#[tokio::main]
async fn main() {
    let ape = User { name: "ape".to_string() };
    let ape_list = ToDoList::new(
        "book",
        vec!["write chapter", "insert code", "draw diagram"]
    );

    let lists = HashMap::from([
        (ape, Vec::from([ape_list])),
    ]);

    let hub = ToDoListHub::new(lists);

    let app = Zettai::new(Arc::new(Mutex::new(hub)));
    println!("Server started at http://localhost:8080/todo/ape/book");
    app.serve(8080).await;
}
