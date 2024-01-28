use super::rocket;
use crate::rocket_builder;
use rocket::http::Status;
use rocket::local::blocking::Client;

#[test]
fn root_test() {
    let client = Client::tracked(rocket_builder()).expect("Valid Rocket instance");
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
}
