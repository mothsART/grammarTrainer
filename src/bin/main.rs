#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use std::io;
use rocket::response::NamedFile;
use std::path::{Path, PathBuf};
use rocket_contrib::{Json};

#[derive(Serialize, Deserialize)]
struct Verbs {
    list: Vec<String>
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/")]
fn admin() -> io::Result<NamedFile> {
    NamedFile::open("static/admin.html")
}

#[get("/<input>", format = "application/json")]
fn verbs(input: String) -> Option<Json<Verbs>> {
    let mut list = Vec::new();
    list.push("prout".to_string());
    list.push("manges".to_string());
    Some(Json(Verbs {
        list: list
    }))
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
    .mount("/", routes![index])
    .mount("/static", routes![files])
    .mount("/admin", routes![admin])
    .mount("/verbs", routes![verbs])
}

fn main() {
    rocket().launch();
}
