use crate::zettai::business::domain::{ListName, ToDoItem, ToDoList, User};
use crate::zettai::business::todolist_fetcher::ToDoListFetcherFromMap;
use std::collections::HashMap;
use std::iter::once;

pub trait ZettaiHub: Send + Sync {
    fn get_todo_list(&self, user: &User, list_name: &ListName) -> Option<&ToDoList>;
    fn get_all_todo_lists(&self, user: &User) -> HashMap<ListName, ToDoList>;
    fn add_item_to_list(&mut self, user: &User, list_name: &ListName, item: &ToDoItem);
}

pub struct ToDoListHub {
    fetcher: ToDoListFetcherFromMap,
}

impl ToDoListHub {
    pub fn new(fetcher: ToDoListFetcherFromMap) -> Self {
        ToDoListHub { fetcher }
    }

    fn replace_item(items: &Vec<ToDoItem>, new_item: &ToDoItem) -> Vec<ToDoItem> {
        items
            .iter()
            .map(|item| item.clone())
            .chain(once(new_item.clone()))
            .collect()
    }
}

impl ZettaiHub for ToDoListHub {
    fn get_todo_list(&self, user: &User, list_name: &ListName) -> Option<&ToDoList> {
        self.fetcher.get(user, list_name)
    }

    fn get_all_todo_lists(&self, user: &User) -> HashMap<ListName, ToDoList> {
        self.fetcher.get_all(user)
    }

    fn add_item_to_list(&mut self, user: &User, list_name: &ListName, item: &ToDoItem) {
        let new_list = match self.fetcher.get(user, list_name) {
            Some(list) => ToDoList {
                list_name: list_name.clone(),
                items: Self::replace_item(&list.items, item),
            },
            None => ToDoList {
                list_name: list_name.clone(),
                items: vec![item.clone()],
            },
        };
        self.fetcher.assign_list_to_user(user, &new_list);
    }
}
