use crate::zettai::business::domain::{ListName, ToDoList, User};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ToDoListFetcherFromMap {
    store: HashMap<User, HashMap<ListName, ToDoList>>,
}

impl ToDoListFetcherFromMap {
    pub fn new(store: HashMap<User, HashMap<ListName, ToDoList>>) -> Self {
        ToDoListFetcherFromMap { store }
    }

    pub fn invoke(&self, user: &User, list_name: &ListName) -> Option<&ToDoList> {
        self.store.get(user)?.get(list_name)
    }

    pub fn assign_list_to_user(&mut self, user: &User, list: &ToDoList) {
        self.store
            .entry(user.clone())
            .and_modify(|list_map| {
                list_map.insert(list.list_name.clone(), list.clone());
            })
            .or_insert_with(|| {
                let mut map = HashMap::new();
                map.insert(list.list_name.clone(), list.clone());
                map
            });
    }
}
