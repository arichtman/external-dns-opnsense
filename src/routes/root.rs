use crate::responses::Response;
#[get("/")]
pub fn get() -> String {
    Response {
        status: rocket::http::Status::Ok,
        message: "Nothing to see here...".into(),
    }
    .to_string()
}
