use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use regex::Regex;
use reqwest::Client;
use crate::zettai::business::domain::{ListName, ToDoItem, ToDoList, User};
use crate::zettai::business::todolist_fetcher::ToDoListFetcherFromMap;
use crate::zettai::business::zettai_hub::ToDoListHub;
use crate::zettai::zettai::Zettai;

// Application For Acceptance Test
#[allow(dead_code)]
pub(super) struct AppForAT {}

#[allow(dead_code)]
impl AppForAT {
    pub(super) async fn get_to_do_list(&self, user: &User, list_name: &str) -> ToDoList {
        let client = Client::new();
        let url = format!("http://localhost:8081/todo/{}/{}", user.name, list_name);
        let response = client.get(&url).send().await.unwrap();

        if response.status().is_success() {
            Self::parse_response(&response.text().await.unwrap())
        } else {
            std::panic!("{}", response.text().await.unwrap());
        }
    }

    pub(super) async fn add_item_to_list(&self, user: &User, item: &str, list_name: &str) {
        let client = Client::new();
        let url = format!("http://localhost:8081/todo/{}/{}", user.name, list_name);
        let form: HashMap<&str, &str> = HashMap::from([("item_name", item)]);
        client.post(&url)
            .form(&form)
            .send().await.unwrap();
    }

    fn parse_response(html: &str) -> ToDoList {
        let name_regex = Regex::new("<h2>(.*?)<").unwrap();
        let list_name = Self::extract_list_name(&name_regex, html);
        let items_regex = Regex::new("<td>(.*?)<").unwrap();
        let items = items_regex.captures_iter(html)
            .map(|cap| ToDoItem { description: cap[1].to_string() })
            .collect();

        ToDoList { list_name: ListName { name: list_name }, items }
    }

    fn extract_list_name(name_regex: &Regex, html: &str) -> String {
        name_regex.captures(html)
            .map(|cap| cap[1].to_string())
            .unwrap_or_default()
    }

    pub(super) async fn start_the_application(&self, fetcher: ToDoListFetcherFromMap) {
        let hub = ToDoListHub::new(fetcher);
        let app = Zettai::new(Arc::new(Mutex::new(hub)));

        tokio::spawn(async move {
            app.serve(8081u16).await;
        });
    }
}