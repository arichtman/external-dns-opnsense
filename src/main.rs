mod responses;
mod routes;
#[cfg(test)]
mod test;

#[macro_use]
extern crate rocket;

use routes::*;

#[launch]
fn rocket_builder() -> _ {
    rocket::build()
        .configure(rocket::Config::figment().merge(("port", 8888)))
        .mount(
            "/",
            routes![
                root::get,
                records::get,
                records::post,
                adjustendpoints::post,
                healthz::get
            ],
        )
}
