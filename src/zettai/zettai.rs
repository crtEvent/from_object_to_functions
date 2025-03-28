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
        let frank = ToDoListOwner { user: User { name: "frank".to_string() } };
        let list_name = "shopping";
        let food_to_buy = vec!("carrots", "apples", "milk");

        start_the_application(&frank.user, &list_name, &food_to_buy);
        frank.can_see_the_list(&list_name, &food_to_buy).await;
    }

    #[tokio::test]
    #[should_panic]
    async fn only_owners_can_see_their_lists() {
        let frank = ToDoListOwner { user: User { name: "frank".to_string() } };
        let bob = ToDoListOwner { user: User { name: "bob".to_string() } };
        let list_name = "shopping";
        let food_to_buy = vec!("carrots", "apples", "milk");

        start_the_application(&frank.user, &list_name, &food_to_buy);
        bob.can_see_the_list(&list_name, &food_to_buy).await;
    }

    trait ScenarioActor {
        fn user(&self) -> &User;
    }

    struct ToDoListOwner {
        user: User,
    }

    impl ScenarioActor for ToDoListOwner {
        fn user(&self) -> &User {
            &self.user
        }
    }

    impl ToDoListOwner {
        pub async fn can_see_the_list(&self, list_name: &str, items: &Vec<&str>) {
            let expected_list = create_list(list_name, items);
            let list = self.get_to_do_list(list_name).await;
            assert_eq!(list.list_name.name, expected_list.list_name.name);
            assert_eq!(list.items, expected_list.items);
        }

        async fn get_to_do_list(&self, list_name: &str) -> ToDoList {
            let client = Client::new();
            let url = format!("http://localhost:8081/todo/{}/{}", self.user.name, list_name);
            let response = client.get(&url).send().await.unwrap();

            if response.status().is_success() {
                parse_response(&response.text().await.unwrap())
            } else {
                panic!("Request failed: {}", response.status());
            }
        }
    }

    fn create_list(list_name: &str, items: &Vec<&str>) -> ToDoList {
        ToDoList {
            list_name:  ListName { name: list_name.to_string() },
            items: items.into_iter()
                .map(|item| ToDoItem { description: item.to_string() })
                .collect(),
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

    fn start_the_application(user: &User, list_name: &str, items: &Vec<&str>) {
        let to_do_list = create_list(list_name, items);
        let mut lists = HashMap::new();
        lists.insert(user.clone(), vec![to_do_list]);

        let app = Zettai::new(lists);
        tokio::spawn(async move {
            app.serve(8081u16).await;
        });
    }
}