use axum::response::Html;

pub fn end_page() -> Html<String> {
    let html = r#"
    <html>
        <body>
            <h1 style="text-align:center; font-size:3em";>Hello Function World!</h1>
        </body>
    </html>
    "#
    .to_string();

    Html(html)
}
