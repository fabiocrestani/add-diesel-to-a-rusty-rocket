// beer.rs
// Author: Fabio Crestani

use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use crate::schema::beers;

#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "beers"]
pub struct Beer {
    pub id: Option<i32>,
    pub name: String,
    pub style: String,
    pub abv: f64
}

impl Beer {
    pub fn create(beer: Beer, connection: &SqliteConnection) -> Beer {
        diesel::insert_into(beers::table)
            .values(&beer)
            .execute(connection)
            .expect("Error creating new beer");
            beers::table.order(beers::id.desc()).first(connection).unwrap()
    }

    pub fn read(connection: &SqliteConnection) -> Vec<Beer> {
        beers::table.order(beers::id).load::<Beer>(connection).unwrap()
    }

    pub fn update(id: i32, beer: Beer, connection: &SqliteConnection) -> bool {
        diesel::update(beers::table.find(id)).set(&beer).execute(connection).is_ok()
    }

    pub fn delete(id: i32, connection: &SqliteConnection) -> bool {
        diesel::delete(beers::table.find(id)).execute(connection).is_ok()
    }
}