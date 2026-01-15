use axum::{extract::State, Json};
use sqlx::SqlitePool;
use shared::Slider;

pub async fn sliders_api(
    State(pool): State<SqlitePool>
) -> Json<Vec<Slider>> {
    let pool = SqlitePool::connect("sqlite://casabaldini.sqlite")
    .await
    .unwrap();
    let sliders = sqlx::query_as!( Slider,"SELECT img, titolo, caption, testo FROM beb_slider")
    .fetch_all(&pool)
    .await
    .unwrap();

    Json(sliders)
}