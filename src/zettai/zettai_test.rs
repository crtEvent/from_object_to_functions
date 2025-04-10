use std::collections::HashMap;
use std::panic;
use std::sync::{Arc, Mutex};
use regex::Regex;
use reqwest::Client;
use crate::zettai::zettai::Zettai;
use crate::zettai::business::domain::{ListName, ToDoItem, ToDoList, User};
use crate::zettai::business::zettai_hub::ToDoListHub;

// Application For Acceptance Test
struct AppForAT {

}

impl AppForAT {
    async fn get_to_do_list(&self, user: &User, list_name: &str) -> ToDoList {
        let client = Client::new();
        let url = format!("http://localhost:8081/todo/{}/{}", user.name, list_name);
        let response = client.get(&url).send().await.unwrap();

        if response.status().is_success() {
            Self::parse_response(&response.text().await.unwrap())
        } else {
            panic!("{}", response.text().await.unwrap());
        }
    }

    fn create_list(&self, list_name: &str, items: &[&str]) -> ToDoList {
        ToDoList {
            list_name:  ListName { name: list_name.to_string() },
            items: items.into_iter()
                .map(|item| ToDoItem { description: item.to_string() })
                .collect(),
        }
    }

    fn parse_response(html: &str) -> ToDoList {
        let name_regex = Regex::new("<h2>(.*?)<").unwrap();
        let list_name = Self::extract_list_name(&name_regex, html);
        let items_regex = Regex::new("<td>(.*?)<").unwrap();
        let items = items_regex.captures_iter(html)
            .map(|cap| ToDoItem { description: cap[1].to_string() })
            .collect();

        ToDoList { list_name: ListName{ name: list_name }, items }
    }

    fn extract_list_name(name_regex: &Regex, html: &str) -> String {
        name_regex.captures(html)
            .map(|cap| cap[1].to_string())
            .unwrap_or_default()
    }

    fn start_the_application(&self, lists: HashMap<User, Vec<ToDoList>>) {
        let hub = ToDoListHub::new(lists);
        let app = Zettai::new(Arc::new(Mutex::new(hub)));
        tokio::spawn(async move {
            app.serve(8081u16).await;
        });
    }
}

struct ToDoListOwner {
    user_name: String,
}

impl ToDoListOwner {
    fn user(&self) -> User {
        User { name: self.user_name.clone() }
    }

    async fn can_see_the_list(&self, expected_list: &ToDoList, app: &AppForAT) {
        let list = app.get_to_do_list(&self.user(), &expected_list.list_name.name).await;
        assert_eq!(list.list_name, expected_list.list_name);
        assert_eq!(list.items, expected_list.items);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn list_owners_can_see_their_lists() {
        let app = AppForAT{};
        let frank = ToDoListOwner { user_name: "frank".to_string() };
        let frank_list = app.create_list(
            "shopping",
            &vec!("carrots", "apples", "milk")
        );

        let map: HashMap<User, Vec<ToDoList>> = HashMap::from([
            (frank.user(), Vec::from([frank_list.clone()])),
        ]);

        app.start_the_application(map);
        frank.can_see_the_list(&frank_list, &app).await;
    }

    #[tokio::test]
    #[should_panic]
    async fn only_owners_can_see_their_lists() {
        let app = AppForAT{};
        let frank = ToDoListOwner { user_name: "frank".to_string() };
        let frank_list = app.create_list(
            "shopping",
            &vec!("carrots", "apples", "milk")
        );
        let bob = ToDoListOwner { user_name: "bob".to_string() };

        let map: HashMap<User, Vec<ToDoList>> = HashMap::from([
            (frank.user(), Vec::from([frank_list.clone()])),
            (bob.user(), Vec::from([])),
        ]);

        app.start_the_application(map);
        bob.can_see_the_list(&frank_list, &app).await;
    }

}