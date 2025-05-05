use crate::zettai::business::domain::{ListName, ToDoItem, ToDoList, User};
use crate::zettai::business::zettai_hub::ZettaiHub;
use axum::extract::Path;
use axum::response::Html;
use std::sync::{Arc, Mutex};

pub fn get_todo_list(
    hub: Arc<Mutex<dyn ZettaiHub>>,
    Path((user_name, list_name)): Path<(String, String)>,
) -> Html<String> {
    let list_data = extract_list_data(user_name, list_name);
    let todo_list = fetch_list_content(hub, &list_data);
    let html = render_html(&list_data.0, &todo_list);
    html
}

fn extract_list_data(user: String, list_name: String) -> (User, ListName) {
    (User { name: user }, ListName { name: list_name })
}

fn fetch_list_content(hub: Arc<Mutex<dyn ZettaiHub>>, list_id: &(User, ListName)) -> ToDoList {
    hub.lock()
        .unwrap()
        .get_todo_list(&list_id.0, &list_id.1)
        .cloned()
        .expect("List unknown")
}

fn render_html(user: &User, todo_list: &ToDoList) -> Html<String> {
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
                    <hr>
                    <h5>Add item to list</h5>
                    <form action="/todo/{}/{}" method="post">
                        <label for="item_name">Item Name: </label>
                        <input type="text" name="item_name">
                        <label for="due_date">Due Date: </label>
                        <input type="date" name="due_date">
                        <label for="status">Status: </label>
                        <select name="status">
                            <option value="todo">Todo</option>
                            <option value="in_progress">InProgress</option>
                            <option value="done">Done</option>
                            <option value="blocked">Blocked</option>
                        </select>
                        <input type="submit" value="Submit">
                    </form>
                </body>
            </html>
            "#,
        todo_list.list_name.name, item_html, user.name, todo_list.list_name.name
    );

    Html(html)
}

fn render_items_to_html(items: &Vec<ToDoItem>) -> String {
    items
        .iter()
        .map(|item| {
            format!(
                r#"<tr><td>{}</td><td>{}</td><td>{}</td></tr>"#,
                item.description,
                item.due_date,
                item.state.as_str()
            )
        })
        .collect::<Vec<_>>()
        .join("")
}
