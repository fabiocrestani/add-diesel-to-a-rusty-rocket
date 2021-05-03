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
use std::thread;
use std::time::Duration;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

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
    json!(r)
}

// Using this function to create a new SqliteConnection for each query to the database
pub fn establish_new_db_connection() -> SqliteConnection {
    let url = "db.db";
    SqliteConnection::establish(&url).expect(&format!("Error connecting to {}", url))
}

// Experimenting with concurrent access to database
fn headache_thread_handler(id: i32, number_of_bottles: u32) {
    for i in 1..number_of_bottles {
        let beer = Beer::get_by_id(&establish_new_db_connection(), id);
        println!("Opening the {}. of {:?}", i+1, beer);
    }
}

// Because working with threads is a headache
#[get("/")]
fn headache(connection: db::Connection) -> JsonValue {
    // First get list of all beers
    let list_of_beers = Beer::read(&connection);
    let length = list_of_beers.len();
    let number_of_bottles = 500;

    // Then, for each beer spawn a thread
    for beer in list_of_beers {
        thread::spawn(move || {
            match beer.id {
                Some(id) => {headache_thread_handler(id, number_of_bottles);},
                None => {println!("ID not found");}
            }
            thread::sleep(Duration::from_millis(2));
        });
    }

    json!({"status": format!("Opened {} beers", (length as u32) * number_of_bottles)})
}

fn main() {
    rocket::ignite()
        .manage(db::connect())
        .mount("/headache", routes![headache])
        .mount("/beer", routes![create, update, delete])
        .mount("/beers", routes![read])
        .launch();
        
}