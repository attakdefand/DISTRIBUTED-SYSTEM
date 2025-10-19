use axum::response::Html;

pub async fn handler() -> Html<&'static str> {
    Html("<h1>Rust Layer System - Layer 1</h1>")
}