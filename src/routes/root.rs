use rocket::response::status::BadRequest;
#[get("/", format = "json")]
pub fn get() -> Result<String, BadRequest<String>> {
    // Ok("Sure, whatever".into())
    Err(BadRequest("no bueno".into()))
}
