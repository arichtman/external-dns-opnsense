mod routes;
use rocket::*;

// use rocket::*;
// #[macro_use]
// extern crate rocket;

#[launch]
pub fn rocket_builder() -> Rocket<Build> {
    rocket::build()
        .configure(rocket::Config::figment().merge(("port", 8888)))
        .mount(
            "/",
            routes![
                routes::root::get,
                routes::records::get,
                routes::records::post,
                routes::adjustendpoints::post,
                routes::healthz::get
            ],
        )
}
