use std::collections::HashMap;
use crate::zettai::business::domain::{ListName, ToDoList, User};

pub trait ZettaiHub: Send + Sync {
    fn get_list(&self, user: &User, list_name: &ListName) -> Option<&ToDoList>;
}

pub struct ToDoListHub {
    lists: HashMap<User, Vec<ToDoList>>,
}

impl ToDoListHub {
    pub fn new(lists: HashMap<User, Vec<ToDoList>>) -> Self {
        ToDoListHub { lists, }
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
}