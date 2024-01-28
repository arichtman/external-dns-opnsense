use crate::responses::Response;
#[get("/healthz")]
pub fn get() -> String {
    Response {
        status: rocket::http::Status::Ok,
        message: "Genki".into(),
    }
    .to_string()
}
