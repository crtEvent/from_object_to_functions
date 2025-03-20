#[derive(Debug, Clone)]
pub struct ToDoList {
    pub list_name: ListName,
    pub items: Vec<ToDoItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
