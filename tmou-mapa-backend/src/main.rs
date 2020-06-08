#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::http::RawStr;
use rocket::{Request, Data};
use rocket::fairing::{Fairing, Info, Kind};


#[get("/game/<secret_phrase>/team")]
fn teams(secret_phrase: &RawStr) -> String {
    format!("Team Info for {}", secret_phrase.as_str())
}

#[get("/game/<secret_phrase>/nodes")]
fn nodes(secret_phrase: &RawStr) -> String {
    format!("Sending nodes for {}", secret_phrase.as_str())
}

#[get("/game/<secret_phrase>/tiles")]
fn tiles(secret_phrase: &RawStr) -> String {
    format!("Sending tiles for {}", secret_phrase.as_str())
}

#[get("/")]
fn index() -> String {
    format!("Become the legend!")
}


fn main() {
    rocket::ignite()
        .mount("/", routes![index, teams, nodes, tiles])
        .attach(PhraseChecker)
        .launch();
}

struct PhraseChecker;

impl Fairing for PhraseChecker {
    fn  info(&self) -> Info {
        Info {
            name: "Phrase Checker",
            kind: Kind::Request
        }
    }

    //TODO: Add response and return 404 for non-matching phrase
    fn on_request(&self, request: &mut Request, _: &Data) {
        println!("Fairing called");
        println!("Tajna fraze je: {:?}", request.uri().segments().nth(1));
    }
}
