use std::collections::HashMap;
use std::panic;
use std::sync::{Arc, Mutex};
use regex::Regex;
use reqwest::Client;
use tokio::sync::OnceCell;
use crate::zettai::zettai::Zettai;
use crate::zettai::business::domain::{ListName, ToDoItem, ToDoList, User};
use crate::zettai::business::zettai_hub::ToDoListHub;

static SINGLE_SERVER_HANDLER: OnceCell<()> = OnceCell::const_new();

// Application For Acceptance Test
struct AppForAT {}

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

    async fn start_the_application(&self, lists: HashMap<User, Vec<ToDoList>>) {
        SINGLE_SERVER_HANDLER
            .get_or_init(|| async {
                let hub = ToDoListHub::new(lists);
                let app = Zettai::new(Arc::new(Mutex::new(hub)));

                tokio::spawn(async move {
                    app.serve(8081u16).await;
                });

                tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            })
            .await;
    }
}

struct ToDoListOwner {
    user_name: String,
}

impl ToDoListOwner {
    fn new(user_name: &str) -> Self {
        ToDoListOwner { user_name: user_name.to_string() }
    }

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
    use once_cell::sync::Lazy;
    use super::*;

    static FRANK: Lazy<ToDoListOwner> = Lazy::new(||
        ToDoListOwner::new("frank"));
    static FRANK_LIST: Lazy<ToDoList> = Lazy::new(||
        ToDoList::new(
            "shopping",
            vec!("carrots", "apples", "milk")
        ));
    static BOB: Lazy<ToDoListOwner> = Lazy::new(||
        ToDoListOwner::new("bob"));
    static BOB_LIST: Lazy<ToDoList> = Lazy::new(||
        ToDoList::new(
            "gardening",
            vec!("fix the fence", "mowing the lawn",)
        ));
    static MAP: Lazy<HashMap<User, Vec<ToDoList>>> = Lazy::new(||
        HashMap::from([
            (FRANK.user(), Vec::from([FRANK_LIST.clone()])),
            (BOB.user(), Vec::from([BOB_LIST.clone()])),
        ]));


    #[tokio::test]
    async fn list_owners_can_see_their_lists() {
        let app = AppForAT {};
        app.start_the_application(MAP.clone()).await;

        FRANK.can_see_the_list(&FRANK_LIST, &app).await;
    }

    #[tokio::test]
    #[should_panic]
    async fn only_owners_can_see_their_lists() {
        let app = AppForAT {};
        app.start_the_application(MAP.clone()).await;

        BOB.can_see_the_list(&FRANK_LIST, &app).await;
    }
}