extern crate diesel;
extern crate hello_rocket;

use self::hello_rocket::*;
use crate::diesel::RunQueryDsl;

fn main() {
    let connection = db::establish_connection();

    // Destroy existing tables
    let _ = diesel::delete(schema::available_parts::table)
        .execute(&connection)
        .unwrap();

    // Create new tables
    let _part = views::create_available_part(
        &connection,
        &"2n5908".to_string(),
        &"Transistor".to_string(),
        &1,
    );

    let results = schema::available_parts::table
        // .filter(quantity)
        // .limit(5)
        .load::<models::AvailablePart>(&connection)
        .expect("Error loading posts");
    println!("Parts {:?}", results);
}
