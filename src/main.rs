#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

use rocket_contrib::json::JsonValue;

#[get("/json")]
fn hello() -> JsonValue {
  json!("Hello world\n")
}

fn main() {
  rocket::ignite().mount("/", routes![hello]).launch();
}
