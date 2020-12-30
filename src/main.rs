#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::http::RawStr;

#[get("/user/<id>")]
fn user(id: usize) -> String {
    format!("you called user() with: {}", id)
}

#[get("/user/<id>", rank = 2)]
fn user_int(id: isize) -> String {
    format!("you called user_int() with: {}", id)
}

#[get("/user/<id>", rank = 3)]
fn user_str(id: &RawStr) -> String {
    format!("you called user_str() with: {}", id)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![user, user_int, user_str])
        .launch();
}
