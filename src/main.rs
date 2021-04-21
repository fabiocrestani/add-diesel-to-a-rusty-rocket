// main.rs
// Author: Fabio Crestani

#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;
//use serde::{Serialize, Deserialize};

mod db;
mod schema;
mod beer;
use beer::Beer;

// Add a new beer
#[post("/", data = "<beer>")]
fn create(beer: Json<Beer>, connection: db::Connection) -> JsonValue {
    let insert = Beer { id: beer.id, ..beer.into_inner() };
    json!(Beer::create(insert, &connection))
}

// Update beer with id /<id>
#[put("/<id>", data = "<beer>")]
fn update(id: i32, beer: Json<Beer>, connection: db::Connection) -> JsonValue {
    let update = Beer { id: Some(id), ..beer.into_inner() };
    json!({
        "success": Beer::update(id, update, &connection)
    })
}

// Delete beer with id /<id>
#[delete("/<id>")]
fn delete(id: i32, connection: db::Connection) -> JsonValue {
    json!({
        "success": Beer::delete(id, &connection)
    })
}

// Returns a list of all beers
#[get("/")]
fn read(connection: db::Connection) -> JsonValue {
    let r = Beer::read(&connection);
    //println!("{:?}", json!(r));
    json!(r)
}

fn main() {
    rocket::ignite()
        .manage(db::connect())
        .mount("/beer", routes![create, update, delete])
        .mount("/beers", routes![read])
        .launch();
}