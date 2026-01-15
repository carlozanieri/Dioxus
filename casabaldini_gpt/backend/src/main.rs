//use axum::{Router, routing::get, response::Html};
use axum::{routing::get, Router};
use tokio::net::TcpListener;
use reqwest;
use std::net::SocketAddr;

use frontend::App;
mod api;
use crate::api::sliders_api;
use sqlx::SqlitePool;


async fn dioxus_app() -> Html<String> {
    Html(dioxus_ssr::render_to_string(App))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/sliders", get(sliders_api));

    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}async fn main() {
    
    let pool = SqlitePool::connect("sqlite://casabaldini.sqlite")
    .await
    .unwrap();

    let app = Router::new()
        .route("/api/sliders", get(sliders_api))
        .route("/*path", get(dioxus_app))
        .with_state(pool)
        .nest_service("/static", tower_http::services::ServeDir::new("static"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 http://{}", addr);}