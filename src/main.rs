#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use std::collections::HashMap;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;

#[get("/")]
fn index() -> Template {
    let mut context = HashMap::new();

    context.insert("test", "test");

    Template::render("index", context)
}

fn main() {
    rocket::ignite()
    .attach(Template::fairing())
    .mount("/assets", StaticFiles::from("./assets"))
    .mount("/", routes![index])
    .launch();
}