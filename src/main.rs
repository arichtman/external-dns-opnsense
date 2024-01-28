mod responses;
mod routes;
#[cfg(test)]
mod test;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket_builder() -> _ {
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
