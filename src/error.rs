use axum::{http::StatusCode, response::IntoResponse, response::Response};

pub enum OPNsenseError {
    AuthFail,
}

impl IntoResponse for OPNsenseError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "OpnSense Auth Error").into_response()
    }
}
