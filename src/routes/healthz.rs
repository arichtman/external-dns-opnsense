#[get("/healthz")]
pub fn get() -> &'static str {
    "{}"
}
