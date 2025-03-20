use std::collections::HashMap;
use crate::zettai::domain::{ListName, ToDoItem, ToDoList, User};

pub fn list_page(lists: &HashMap<User, Vec<ToDoList>>, user: String, list_name: String) -> String {
    let list_data = extract_list_data(user, list_name);
    let todo_list = fetch_list_content(&lists, &list_data);
    let html = render_html(&todo_list);
    html
}

fn extract_list_data(user: String, list_name: String) -> (User, ListName) {
    (User { name: user }, ListName { name: list_name })
}

fn fetch_list_content(lists: &HashMap<User, Vec<ToDoList>>, list_id: &(User, ListName)) -> ToDoList {
    lists.get(&list_id.0)
        .and_then(|lists|
            lists.iter().find(|l|
                l.list_name.name == list_id.1.name))
        .cloned()
        .expect("List unknown")
}

fn render_html(todo_list: &ToDoList) -> String {
    let item_html = render_items(&todo_list.items);
    format!(
        r#"
            <html>
                <body>
                    <h1>Zettai</h1>
                    <h2>{}</h2>
                    <table>
                        <tbody>{}</tbody>
                    </table>
                </body>
            </html>
            "#,
        todo_list.list_name.name, item_html
    )
}

fn render_items(items: &Vec<ToDoItem>) -> String {
    items.iter()
        .map(|item| format!(r#"<tr><td>{}</td></tr>"#, item.description))
        .collect::<Vec<_>>()
        .join("")
}
