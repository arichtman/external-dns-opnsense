use crate::responses::Response;
#[get("/records")]
pub fn get() -> String {
    Response {
        status: rocket::http::Status::Ok,
        message: "Hmm yes records, much wow".into(),
    }
    .to_string()
}
#[post("/records")]
pub fn post() -> String {
    Response {
        status: rocket::http::Status::NotImplemented,
        message: "No touching!".into(),
    }
    .to_string()
}
