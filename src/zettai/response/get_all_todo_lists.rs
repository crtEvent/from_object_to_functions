use crate::zettai::business::domain::{ToDoList, User};
use crate::zettai::business::zettai_hub::ZettaiHub;
use axum::extract::Path;
use axum::response::Html;
use std::sync::{Arc, Mutex};

pub fn get_all_todo_lists(
    hub: Arc<Mutex<dyn ZettaiHub>>,
    Path(user_name): Path<String>,
) -> Html<String> {
    let user = User { name: user_name };
    let todo_lists = fetch_lists_content(hub, &user);
    let html = render_html(&user, &todo_lists);
    html
}

fn fetch_lists_content(hub: Arc<Mutex<dyn ZettaiHub>>, user: &User) -> Vec<ToDoList> {
    let mut todo_lists: Vec<ToDoList> = hub
        .lock()
        .unwrap()
        .get_all_todo_lists(&user)
        .values()
        .cloned()
        .collect();
    todo_lists.sort_by_key(|todo| todo.list_name.clone());

    todo_lists
}

fn render_html(user: &User, todo_lists: &Vec<ToDoList>) -> Html<String> {
    let lists_html = render_lists_to_html(todo_lists);
    let html = format!(
        r#"
            <html>
                <body>
                    <h1>Zettai</h1>
                    <h2>{}'s Todo Lists</h2>
                    <table>
                        <tbody>{}</tbody>
                    </table>
                </body>
            </html>
            "#,
        user.name, lists_html
    );

    Html(html)
}

fn render_lists_to_html(todo_lists: &Vec<ToDoList>) -> String {
    todo_lists
        .iter()
        .map(|todo| format!(r#"<tr><td>{}</td></tr>"#, todo.list_name.name))
        .collect::<Vec<String>>()
        .join("")
}
