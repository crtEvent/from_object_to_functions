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
