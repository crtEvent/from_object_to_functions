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
    let lists_html = render_lists_to_html(user, todo_lists);
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
                <hr>
                <h5>Create new to-do List</h5>
                <form action="/todo/{}" method="post">
                    <label for="list_name">List Name: </label>
                    <input type="text" name="list_name">
                    <input type="submit" value="Submit">
                </form>
            </html>
            <script>
                document.querySelectorAll('tr[data-href]').forEach(tr => {{
                    tr.addEventListener('click', () => {{
                        window.location.href = tr.dataset.href;
                    }});
                    tr.style.cursor = 'pointer';
                }});
            </script>
            "#,
        user.name, lists_html, user.name
    );

    Html(html)
}

fn render_lists_to_html(user: &User, todo_lists: &Vec<ToDoList>) -> String {
    todo_lists
        .iter()
        .map(|todo| {
            format!(
                r#"<tr data-href="/todo/{}/{}"><td>{}</td></tr>"#,
                user.name, todo.list_name.name, todo.list_name.name
            )
        })
        .collect::<Vec<String>>()
        .join("")
}
