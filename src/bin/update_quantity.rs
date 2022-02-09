extern crate diesel;
extern crate hello_rocket;

use self::diesel::prelude::*;
use self::hello_rocket::*;
use self::models::AvailablePart;
use std::env::args;

fn main() {
    use hello_rocket::schema::available_parts::dsl::{available_parts, quantity};

    let id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i64>()
        .expect("Invalid ID");
    let connection = establish_connection();

    let part = diesel::update(available_parts.find(id))
        .set(quantity.eq(5))
        .get_result::<AvailablePart>(&connection)
        .expect(&format!("Unable to find post {}", id));
    println!("Published part {}", part.part_name);
}
