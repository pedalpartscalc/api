extern crate diesel;
extern crate hello_rocket;

use self::diesel::prelude::*;
use self::hello_rocket::*;
use std::env::args;

fn main() {
    use hello_rocket::schema::available_parts::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = establish_connection();
    let num_deleted = diesel::delete(available_parts.filter(part_name.like(pattern)))
        .execute(&connection)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
}
