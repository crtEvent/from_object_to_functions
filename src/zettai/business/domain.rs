#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToDoList {
    pub list_name: ListName,
    pub items: Vec<ToDoItem>,
}
impl ToDoList {
    pub fn new(list_name: &str, items: Vec<&str>) -> Self {
        ToDoList {
            list_name: ListName { name: list_name.to_string() },
            items: items.into_iter()
                .map(|item|
                    ToDoItem { description: item.to_string() }
                )
                .collect()
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ListName {
    pub name: String,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct User {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToDoItem {
    pub description: String,
}

// #[derive(Debug, Clone, Copy)]
// pub enum ToDoStatus {
//     Todo,
//     InProgress,
//     Done,
//     Blocked,
// }
