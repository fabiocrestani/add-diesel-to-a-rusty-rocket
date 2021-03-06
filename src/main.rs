// main.rs
// Author: Fabio Crestani

#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;
use rocket::response::NamedFile;
//use serde::{Serialize, Deserialize};
use std::thread;
use std::panic;
use std::process;
use std::time::Duration;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::connection::SimpleConnection;
extern crate csv;
use csv::Writer;

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

// Returns a list of all beers inside a CSV file
#[get("/")]
fn downloadCsv(connection: db::Connection) -> Option<NamedFile> {
    let r = Beer::read(&connection);
    let mut w = match Writer::from_path("test.csv") {
    	Ok(mut csv_w) => {
			csv_w.write_record(&["id", "name", "style", "abv"]).unwrap_or(());
			for beer in &r {
				let id = match beer.id {
					Some(id) => id.to_string(),
					None => "".to_string()
				};
				let name = beer.name.clone();
				let style = beer.style.clone();
				csv_w.write_record(&[id, name, style, beer.abv.to_string()]).unwrap_or(());
			}

			csv_w.flush().unwrap_or(());
    	},
    	Err(_) => { println!("Error creating csv file"); }
	};
	
	
  
    NamedFile::open("test.csv").ok()
}


// Using this function to create a new SqliteConnection for each query to the database
pub fn establish_new_db_connection() -> SqliteConnection {
    let url = "db.db";
    let connection: SqliteConnection = SqliteConnection::establish(&url).expect(&format!("Error connecting to {}", url));

    // Comment this line to see the program panic
    //match connection.batch_execute("PRAGMA busy_timeout=1000; PRAGMA journal_mode=WAL") {
    match connection.batch_execute("PRAGMA busy_timeout=1000;") {
        Ok(_r) => (),
        Err(r) => println!("Could not set PRAGMA: {:?}", r)
    }
    connection
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
    let number_of_bottles = 700;

    // Then, for each beer spawn a thread
    for beer in list_of_beers {
        thread::spawn(move || {
            match beer.id {
                Some(id) => {headache_thread_handler(id, number_of_bottles);},
                None => {println!("ID not found");}
            }
            thread::sleep(Duration::from_millis(1));
        });
    }

    // One detail here: I am not waiting for all the threads to finish to send the response
    // This could be done by calling .join() for every thread
    json!({"status": format!("Opened {} beers", (length as u32) * number_of_bottles)})
}

fn main() {
    // For debugging only, stopping everything when one thread panicked
    // take_hook() returns the default hook in case when a custom one is not set
    let orig_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        // invoke the default handler and exit the process
        orig_hook(panic_info);
        process::exit(1);
    }));

    rocket::ignite()
        .manage(db::connect())
        .mount("/headache", routes![headache])
        .mount("/beer", routes![create, update, delete])
        .mount("/beers", routes![read])
        .mount("/csv", routes![downloadCsv])
        .launch();
        
}
