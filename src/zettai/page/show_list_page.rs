use std::sync::{Arc, Mutex};
use axum::extract::Path;
use axum::response::Html;
use crate::zettai::business::domain::{ListName, ToDoItem, ToDoList, User};
use crate::zettai::business::zettai_hub::ZettaiHub;

pub fn list_page(
    hub: Arc<Mutex<dyn ZettaiHub>>,
    Path((user_name, list_name)): Path<(String, String)>
) -> Html<String> {
    let list_data = extract_list_data(user_name, list_name);
    let todo_list = fetch_list_content(hub, &list_data);
    let html = render_html(&todo_list);
    html
}

fn extract_list_data(user: String, list_name: String) -> (User, ListName) {
    (User { name: user }, ListName { name: list_name })
}

fn fetch_list_content(hub: Arc<Mutex<dyn ZettaiHub>>, list_id: &(User, ListName)) -> ToDoList {
    hub.lock().unwrap()
        .get_list(&list_id.0, &list_id.1)
        .cloned()
        .expect("List unknown")
}

fn render_html(todo_list: &ToDoList) -> Html<String> {
    let item_html = render_items_to_html(&todo_list.items);
    let html = format!(
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
    );

    Html(html)
}

fn render_items_to_html(items: &Vec<ToDoItem>) -> String {
    items.iter()
        .map(|item|
            format!(r#"<tr><td>{}</td><td>{}</td><td>{}</td></tr>"#,
                    item.description, item.due_date, item.state.as_str())
        )
        .collect::<Vec<_>>()
        .join("")
}
