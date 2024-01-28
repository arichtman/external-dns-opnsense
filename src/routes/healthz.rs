use axum::response::IntoResponse;

pub async fn healthz_get() -> impl IntoResponse {
    "Genki"
}
