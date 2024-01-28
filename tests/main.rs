#[path = "../src/main.rs"]
mod external_dns_opnsense;
use ::external_dns_opnsense::rocket_builder;
use rocket::http::Status;
use rocket::local::blocking::Client;

#[macro_use]
extern crate rocket;
#[test]
fn root_get() {
    let client = Client::tracked(rocket_builder()).expect("Valid Rocket instance");
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
}
