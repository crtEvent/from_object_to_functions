mod exercises;
mod zettai;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::zettai::zettai::Zettai;
use crate::zettai::business::domain::{ListName, ToDoItem, ToDoList, User};
use crate::zettai::business::zettai_hub::ToDoListHub;

#[tokio::main]
async fn main() {
    let items: Vec<&str>  = vec!("write chapter", "insert code", "draw diagram");
    let to_do_list = ToDoList {
        list_name: ListName { name: "book".to_string() },
        items: items.into_iter()
            .map(|item| ToDoItem { description: item.to_string() })
            .collect(),
    };
    let mut lists: HashMap<User, Vec<ToDoList>> = HashMap::new();
    lists.insert(User { name: "ape".to_string() }, vec![to_do_list]);

    let hub = ToDoListHub::new(lists);

    let app = Zettai::new(Arc::new(Mutex::new(hub)));
    println!("Server started at http://localhost:8080/todo/ape/book");
    app.serve(8080).await;
}
