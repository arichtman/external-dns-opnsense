use axum::response::IntoResponse;

pub async fn root_get() -> impl IntoResponse {
    "root get!"
}
