#[macro_use]
extern crate rocket;

#[get("/")]
fn root() -> &'static str {
    "{}"
}
#[get("/records")]
fn records() -> &'static str {
    "{}"
}
#[post("/records")]
fn post_records() -> &'static str {
    "{}"
}
#[post("/adjustendpoints")]
fn adjustendpoints() -> &'static str {
    "{}"
}
#[get("/healthz")]
fn healthz() -> &'static str {
    "{}"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .configure(rocket::Config::figment().merge(("port", 8888)))
        .mount(
            "/",
            routes![root, post_records, records, adjustendpoints, healthz],
        )
}
