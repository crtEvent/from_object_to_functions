mod exercises {
    pub mod ch01 {
        pub mod prac1_2;
        pub mod prac1_3;
    }
}

use warp::Filter;

#[tokio::main]
async fn main() {

    let end_route = warp::path::end()
        .map(end_page);

    let show_list_route = warp::path!("todo" / String / String)
        .map(|user: String, list: String| show_list(user, list));

    let routes = end_route
        .or(show_list_route);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 8080))
        .await;
}

fn end_page() -> impl warp::Reply {
    let html_page = r#"
    <html>
        <body>
            <h1 style="text-align:center; font-size:3em";>Hello Function World!</h1>
        </body>
    </html>
    "#;

    warp::reply::html(html_page)
}

fn show_list(user: String, list: String) -> impl warp::Reply {
    let html_page = format!(
        r#"
        <html>
            <body>
                <h1>Zettai</h1>
                <p>Here is the list <b>{}</b> of user <b>{}</b></p>
            </body>
        </html>
        "#,
        list, user
    );

    warp::reply::html(html_page)
}