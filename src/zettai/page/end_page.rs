pub fn end_page() -> String {
    let html = r#"
    <html>
        <body>
            <h1 style="text-align:center; font-size:3em";>Hello Function World!</h1>
        </body>
    </html>
    "#.to_string();
    html
}
