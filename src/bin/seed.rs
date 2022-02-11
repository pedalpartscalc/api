extern crate diesel;
extern crate hello_rocket;
use crate::diesel::RunQueryDsl;

use self::hello_rocket::schema::available_parts::dsl::*;
use self::hello_rocket::*;

fn main() {
    let connection = establish_connection();

    // Destroy existing tables
    let _ = diesel::delete(available_parts)
        .execute(&connection)
        .unwrap();

    // Create new tables
    let _part = create_available_part(
        &connection,
        &"2n5908".to_string(),
        &"Transistor".to_string(),
        &1,
    );

    let results = available_parts
        // .filter(quantity)
        // .limit(5)
        .load::<models::AvailablePart>(&connection)
        .expect("Error loading posts");
    println!("Parts {:?}", results);
}
