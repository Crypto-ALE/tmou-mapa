#[allow(unused)]
use rocket::http::Status;
#[allow(unused)]
use rocket::local::Client;

use crate::rocket;

#[test]
fn team_without_cookie_and_phrase_is_redirected() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::TemporaryRedirect);
}

#[test]
fn phrase_are_guarded_by_admin() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let response = client.get("/non-existing-phrase").dispatch();
    assert_eq!(response.status(), Status::Unauthorized);
}

#[test]
fn admin_is_protected_by_password() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let response = client.get("/admin").dispatch();
    assert_eq!(response.status(), Status::Unauthorized);
}
