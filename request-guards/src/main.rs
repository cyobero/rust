#![feature(proc_macro_hygiene, decl_macro, never_type)]

#[macro_use] extern crate rocket;

use rocket::request::{self, Request, FromRequest};
use rocket::outcome::Outcome::*;

#[derive(Debug)]
struct HeaderCount(usize);

impl<'a, 'r> FromRequest<'a, 'r> for HeaderCount {
    type Error = !;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, !> {
        Success(HeaderCount(request.headers().len()))
    }
}

#[get("/")]
fn header_count(header_count: HeaderCount) -> String {
    format!("Your request contained {} headers!", header_count.0)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![header_count])
}


fn main() {
    rocket().launch();
}
