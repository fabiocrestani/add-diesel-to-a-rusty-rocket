// db.rs
// Author: Fabio Crestani

use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use diesel::sqlite::SqliteConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

// Points to database file
static DATABASE_URL: &str = "db.db";

// Initialize the database pool
pub fn connect() -> SqlitePool {
    let manager = ConnectionManager::<SqliteConnection>::new(DATABASE_URL);
    Pool::new(manager).expect("Failed to create pool")
}

// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct Connection(pub PooledConnection<ConnectionManager<SqliteConnection>>);

// Attempts to retrieve a single connection from the managed database pool. If
// no pool is currently managed, fails with an `InternalServerError` status. If
// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for Connection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<SqlitePool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Connection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

impl Deref for Connection {
    type Target = SqliteConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}