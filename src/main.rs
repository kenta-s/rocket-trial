#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod other {
    #[get("/world")]
    pub fn world() -> &'static str {
        "Hello, world!"
    }
}

#[get("/hello")]
pub fn hello() -> &'static str {
    "Hello, outside world!"
}

fn main() {
    // /hello/hello
    // /hello/world
    rocket::ignite().mount("/hello", routes![hello, other::world]).launch();
}
