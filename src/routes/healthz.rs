use rocket::get;
#[get("/healthz")]
pub fn get() -> &'static str {
    "{}"
}
