extern crate diesel;
extern crate hello_rocket;

use self::hello_rocket::*;
use std::io::stdin;

fn main() {
    let connection = establish_connection();

    println!("What would you like your part name to be?");
    let mut part_name = String::new();
    stdin().read_line(&mut part_name).unwrap();
    let part_name = &part_name[..(part_name.len() - 1)]; // Drop the newline character
    println!("\nWhat would you like your part kind to be?");
    let mut part_kind = String::new();
    stdin().read_line(&mut part_kind).unwrap();
    let part_kind = &part_kind[..(part_kind.len() - 1)]; // Drop the newline character
    println!("\nHow many of those do you have?");
    let mut quantity_str = String::new();
    stdin().read_line(&mut quantity_str).unwrap();
    let quantity_str = &quantity_str[..(quantity_str.len() - 1)]; // Drop the newline character
    let quantity: i32 = quantity_str.parse::<i32>().unwrap();

    let part = create_available_part(&connection, part_name, part_kind, &quantity);
    println!("\nSaved draft {} with id {}", part_name, part.id);
}
