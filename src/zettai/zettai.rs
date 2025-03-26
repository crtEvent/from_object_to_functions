use std::collections::HashMap;
use warp::Filter;
use warp::filters::BoxedFilter;
use warp::reply::Html;
use crate::zettai::domain::{ToDoList, User};
use crate::zettai::page::end_page::end_page;
use crate::zettai::page::show_list_page::list_page;

pub struct Zettai {
    lists: HashMap<User, Vec<ToDoList>>,
}

impl Zettai {
    pub fn new(lists: HashMap<User, Vec<ToDoList>>) -> Self {
        Zettai { lists }
    }

    fn create_response(html: String) -> Html<String> {
        warp::reply::html(html)
    }

    pub async fn serve(self, port: u16) {
        let end_page_route = warp::path::end()
            .map(move || Self::create_response(end_page()));

        let lists = self.lists.clone();
        let list_page_route = warp::path!("todo" / String / String)
            .map(move |user: String, list_name: String|
                Self::create_response(list_page(&lists, user, list_name))
            );

        let routes: BoxedFilter<(Html<String>,)> = end_page_route
            .or(list_page_route)
            .unify()
            .boxed();

        warp::serve(routes)
            .run(([127, 0, 0, 1], port))
            .await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;
    use std::collections::HashMap;
    use reqwest::Client;
    use crate::zettai::domain::{ListName, ToDoItem};

    #[tokio::test]
    async fn list_owners_can_see_their_lists() {
        let user = User { name: "frank".to_string() };
        let list_name = ListName { name: "shopping".to_string() };
        let food_to_buy: Vec<ToDoItem> = vec!("carrots", "apples", "milk").into_iter()
            .map(|item| ToDoItem { description: item.to_string() })
            .collect();

        start_the_application(&user, &list_name, &food_to_buy);
        let list = get_to_do_list(&user, &list_name).await;

        assert_eq!(list.list_name.name, list_name.name);
        assert_eq!(list.items, food_to_buy);
    }

    #[tokio::test]
    #[should_panic]
    async fn only_owners_can_see_their_lists() {
        let owner = User { name: "frank".to_string() };
        let another_user = User { name: "bob".to_string() };
        let list_name = ListName { name: "shopping".to_string() };
        let food_to_buy: Vec<ToDoItem> = vec!("carrots", "apples", "milk").into_iter()
            .map(|item| ToDoItem { description: item.to_string() })
            .collect();

        start_the_application(&owner, &list_name, &food_to_buy);
        let _list = get_to_do_list(&another_user, &list_name).await;
    }

    async fn get_to_do_list(user: &User, list_name: &ListName) -> ToDoList {
        let client = Client::new();
        let url = format!("http://localhost:8081/todo/{}/{}", user.name, list_name.name);
        let response = client.get(&url).send().await.unwrap();

        if response.status().is_success() {
            parse_response(&response.text().await.unwrap())
        } else {
            panic!("Request failed: {}", response.status());
        }
    }

    fn parse_response(html: &str) -> ToDoList {
        let name_regex = Regex::new("<h2>(.*?)<").unwrap();
        let list_name = extract_list_name(&name_regex, html);
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

    fn start_the_application(user: &User, list_name: &ListName, items: &Vec<ToDoItem>) {
        let to_do_list = ToDoList {
            list_name: list_name.clone(),
            items: items.clone(),
        };
        let mut lists = HashMap::new();
        lists.insert(user.clone(), vec![to_do_list]);

        let app = Zettai::new(lists);
        tokio::spawn(async move {
            app.serve(8081u16).await;
        });
    }
}