use axum::{
    extract::Query,
    response::Html,
    routing::get,
    Router
};
use rand::{thread_rng, Rng};
use serde::Deserialize;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize)]
struct RnageParameters {
    start: usize,
    end: usize,
}

async fn handler(Query(range): Query<RnageParameters>) -> Html<String> {
    if range.start < range.end {
        let random_number = thread_rng().gen_range(range.start .. range.end);
        Html(format!("<h1>Random Number: {}</h1>", random_number))
    } else {
        Html("<h1>error!</h1>".to_string())
    }
}
