use crate::zettai::business::domain::{ListName, ToDoList, User};
use crate::zettai::business::todolist_fetcher::ToDoListFetcherFromMap;
use crate::zettai::business::zettai_hub::ToDoListHub;
use crate::zettai::test::tooling::parser::get_all_todo_lists_parser;
use crate::zettai::test::tooling::parser::get_todo_list_parser;
use crate::zettai::zettai::Zettai;
use chrono::Local;
use reqwest::Client;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Application For Acceptance Test
#[allow(dead_code)]
pub(crate) struct AppForAT {}

#[allow(dead_code)]
impl AppForAT {
    pub(crate) async fn get_todo_list(&self, user: &User, list_name: &str) -> ToDoList {
        let client = Client::new();
        let url = format!("http://localhost:8081/todo/{}/{}", user.name, list_name);
        let response = client.get(&url).send().await.unwrap();

        if response.status().is_success() {
            get_todo_list_parser::parse_response(&response.text().await.unwrap())
        } else {
            std::panic!("{}", response.text().await.unwrap());
        }
    }

    pub(crate) async fn get_all_todo_lists(&self, user: &User) -> Vec<ListName> {
        let client = Client::new();
        let url = format!("http://localhost:8081/todo/{}", user.name);
        let response = client.get(&url).send().await.unwrap();

        if response.status().is_success() {
            get_all_todo_lists_parser::parse_response(&response.text().await.unwrap())
        } else {
            std::panic!("{}", response.text().await.unwrap());
        }
    }

    pub(crate) async fn add_item_to_list(&self, user: &User, item: &str, list_name: &str) {
        let client = Client::new();
        let url = format!("http://localhost:8081/todo/{}/{}", user.name, list_name);
        let due_date = Local::now().date_naive().to_string();
        let form: HashMap<&str, &str> = HashMap::from([
            ("item_name", item),
            ("due_date", &due_date),
            ("status", "Todo"),
        ]);
        client.post(&url).form(&form).send().await.unwrap();
    }

    pub(crate) async fn start_the_application(&self, fetcher: ToDoListFetcherFromMap) {
        let hub = ToDoListHub::new(fetcher);
        let app = Zettai::new(Arc::new(Mutex::new(hub)));

        tokio::spawn(async move {
            app.serve(8081u16).await;
        });
    }
}
