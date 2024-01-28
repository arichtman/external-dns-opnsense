use serde::Serialize;

#[cfg(test)]
mod test;

#[macro_use]
extern crate rocket;

#[derive(Serialize)]
struct ResponseBody {
    key: String,
    another_key: u8,
}

#[get("/")]
fn root() -> String {
    let response = ResponseBody {
        key: "value".into(),
        another_key: 5,
    };
    serde_json::to_string(&response).unwrap()
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
