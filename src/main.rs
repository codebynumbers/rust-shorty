#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use std::hash::Hasher;
use rocket::request::Form;
use pruefung::fnv::fnv32::Fnv32a;

#[derive(FromForm)]
struct Site {
    url: String
}


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<tag>")]
fn expand(tag: String) -> String {
    tag
}

#[get("/data/shorten?<site..>")]
fn shorten(site: Option<Form<Site>>) -> String {
    if let Some(site) = site {
        let mut hasher = Fnv32a::default();
        hasher.write(site.url.as_bytes());
        let hash = hasher.finish();
        format!("{:x}", hash)
    } else {
        String::from("error")
    }
}


fn main() {
    rocket::ignite().mount("/", routes![index, expand, shorten]).launch();
}
