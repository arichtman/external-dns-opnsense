use serde::Serialize;
use serde_json::to_string as json;

#[derive(Serialize)]
pub struct Response {
    pub status: rocket::http::Status,
    pub message: String,
}

// TODO: Check if this is the right trait to implement for into()
impl ToString for Response {
    fn to_string(&self) -> String {
        json(self).unwrap_or(
            json(&Response {
                status: rocket::http::Status::InternalServerError,
                message: "We fucked up".into(),
            })
            .unwrap(),
        )
    }
}
