use axum::response::{IntoResponse, Response};

pub fn get_error_page(message: &str) -> Response {
    let html = format!("<h1>Bad Request</h1><p>{}</p>", message);

    Response::builder()
        .status(400)
        .body(html)
        .unwrap()
        .into_response()
}
