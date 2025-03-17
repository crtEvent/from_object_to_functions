mod exercises {
    pub mod ch01 {
        pub mod prac1_2;
        pub mod prac1_3;
    }
}

use warp::Filter;

#[tokio::main]
async fn main() {
    // HTML 페이지 정의
    let html_page = r#"
    <html>
        <body>
            <h1 style="text-align:center; font-size:3em";>Hello Function World!</h1>
        </body>
    </html>
    "#;

    let handler = warp::path::end()
        .map(move || warp::reply::html(html_page));

    warp::serve(handler)
        .run(([127, 0, 0, 1], 8080))
        .await;
}

