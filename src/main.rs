#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod routes;

#[get("/world")]
fn index() -> &'static str {
    "Hello, world"
}

fn main() {
    rocket::ignite().mount("/api", routes![index, routes::another, routes::opa]).launch();
}
