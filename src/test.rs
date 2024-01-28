use super::rocket;
use rocket::http::Status;
use rocket::local::blocking::Client;

#[test]
fn root_test() {
    let client = Client::tracked(rocket()).expect("Valid Rocket instance");
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
}
