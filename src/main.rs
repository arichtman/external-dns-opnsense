// TODO: Remove for production
#![allow(unused)]

use axum::response::IntoResponse;
use axum::response::IntoResponseParts;
use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").await.unwrap();
    axum::serve(listener, app().into_make_service())
        .await
        .unwrap();
    // let address = SocketAddr::from(([127, 0, 0, 1], 8888));
    // axum::Server::bind(&address)
    //     .serve(routes.into_make_service())
    //     .await
    //     .unwrap();
}

async fn root_get() -> impl IntoResponse {
    "root get!"
}

fn app() -> Router {
    Router::new().route("/", get(root_get))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::extract::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn get_root() {
        let app = app();
        let response = app.oneshot(Request::builder().uri("/").body(Body::empty()).unwrap());
    }
}
