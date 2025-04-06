mod exercises;
mod zettai;

use std::collections::HashMap;
use crate::zettai::domain::{ListName, ToDoItem, ToDoList, User};
use crate::zettai::zettai::Zettai;

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

    let app = Zettai::new(lists);
    app.serve(8080).await;
    println!("Server started at http://localhost:8080/todo/ape/book");
}
