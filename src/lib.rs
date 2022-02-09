#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

use self::models::{AvailablePart, NewAvailablePart};

pub fn create_available_part<'a>(
    conn: &PgConnection,
    part_name: &'a str,
    part_kind: &'a str,
    quantity: &'a i32,
) -> AvailablePart {
    use schema::available_parts;

    let new_part = NewAvailablePart {
        part_name: part_name,
        part_kind: part_kind,
        quantity: quantity,
        owner_id: &1,
    };

    diesel::insert_into(available_parts::table)
        .values(&new_part)
        .get_result(conn)
        .expect("Error saving new part")
}
