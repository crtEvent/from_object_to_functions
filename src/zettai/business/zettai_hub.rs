use std::collections::HashMap;
use crate::zettai::business::domain::{ListName, ToDoItem, ToDoList, User};

pub trait ZettaiHub: Send + Sync {
    fn get_list(&self, user: &User, list_name: &ListName) -> Option<&ToDoList>;
    fn add_item_to_list(
        &mut self, user: &User, list_name: &ListName, item: &ToDoItem,
    ) -> Result<(), String>;
}

pub struct ToDoListHub {
    lists: HashMap<User, Vec<ToDoList>>,
}

impl ToDoListHub {
    pub fn new(lists: HashMap<User, Vec<ToDoList>>) -> Self {
        ToDoListHub { lists }
    }
}

impl ZettaiHub for ToDoListHub {
    fn get_list(&self, user: &User, list_name: &ListName) -> Option<&ToDoList> {
        self.lists
            .get(user)
            .and_then(|lists|
                lists.iter()
                    .find(|list|
                        &list.list_name.name == &list_name.name
                    )
            )
    }

    fn add_item_to_list(
        &mut self, user: &User, list_name: &ListName, item: &ToDoItem
    ) -> Result<(), String> {
        match self.lists.get_mut(user) {
            Some(lists) => {
                match lists.iter_mut().find(|list| &list.list_name == list_name) {
                    Some(list) => {
                        list.items.push(item.clone());
                        Ok(())
                    },
                    None => Err(format!("List '{}' not found", list_name.name))
                }
            }
            None => Err(format!("User '{}' not found", user.name))
        }
    }
}