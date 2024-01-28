use crate::responses::Response;
#[post("/adjustendpoints")]
pub fn post() -> String {
    Response {
        status: rocket::http::Status::NotImplemented,
        message: "Coming Soon :TM:".into(),
    }
    .to_string()
}
