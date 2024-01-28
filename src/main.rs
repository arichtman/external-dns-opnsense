// TODO: Remove for production
#![allow(unused)]

use tokio::net::TcpListener;

mod error;
mod routes;
#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("[::]:8888").await.unwrap();
    axum::serve(listener, routes::app().into_make_service())
        .await
        .unwrap();
}

// Ref: https://github.com/tokio-rs/axum/blob/main/examples/testing/src/main.rs
#[cfg(test)]
mod tests {
    use super::*;
    use crate::routes::app;
    use axum::body::Body;
    use axum::extract::Request;
    use axum::http::StatusCode;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn get_root() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
