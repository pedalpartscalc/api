extern crate diesel;
extern crate hello_rocket;

use self::diesel::prelude::*;
use self::hello_rocket::*;
use self::models::*;

fn main() {
    use hello_rocket::schema::available_parts::dsl::*;

    let connection = establish_connection();
    let results = available_parts
        // .filter(quantity)
        .limit(5)
        .load::<AvailablePart>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for part in results {
        println!("{}", part.part_name);
        println!("----------\n");
        println!("{}", part.quantity);
    }
}
