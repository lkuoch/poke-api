use axum::{
    body::StreamBody,
    http::{header, StatusCode},
    response::{Headers, IntoResponse},
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tokio_util::io::ReaderStream;

async fn handler() -> impl IntoResponse {
    let file = match tokio::fs::File::open("api/v2/index.json").await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };

    let stream = ReaderStream::new(file);

    let body = StreamBody::new(stream);

    let headers = Headers([(header::CONTENT_TYPE, "text/json; charset=utf-8")]);

    Ok((headers, body))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));
    let addr = SocketAddr::from(([127, 0, 0, 1], 5000));

    println!("Hello, world!");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
