use warp::Filter;
use warp::filters::BoxedFilter;
use warp::reply::Html;

pub struct Zettai {
    routes: BoxedFilter<(Html<String>,)>
}

impl Zettai {
    pub fn new() -> Self {

        let end_route = warp::path::end()
            .map(Self::end_page);

        let show_list_route = warp::path!("todo" / String / String)
            .map(|user: String, list: String| Self::show_list_page(user, list));

        let routes = end_route
            .or(show_list_route)
            .unify()
            .boxed();

        Zettai { routes }
    }

    fn end_page() -> Html<String> {
        let html_page = r#"
    <html>
        <body>
            <h1 style="text-align:center; font-size:3em";>Hello Function World!</h1>
        </body>
    </html>
    "#.to_string();

        warp::reply::html(html_page)
    }

    fn show_list_page(user: String, list: String) -> Html<String> {
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

    pub async fn serve(self) {
        warp::serve(self.routes)
            .run(([127, 0, 0, 1], 8080))
            .await;
    }
}