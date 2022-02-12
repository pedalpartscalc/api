use crate::schema::available_parts;

use crate::models::{AvailablePart, NewAvailablePart};
use diesel::prelude::*;

pub fn create_available_part<'a>(
    conn: &PgConnection,
    part_name: &'a str,
    part_kind: &'a str,
    quantity: i32,
) -> AvailablePart {
    let new_part = NewAvailablePart {
        part_name: part_name,
        part_kind: part_kind,
        quantity: quantity,
        owner_id: 1,
    };

    diesel::insert_into(available_parts::table)
        .values(&new_part)
        .get_result(conn)
        .expect("Error saving new part")
}

pub fn list_parts() -> String {
    let connection = crate::db::establish_connection();

    let results = available_parts::table
        .load::<AvailablePart>(&connection)
        .expect("Error loading parts");
    let serialized = serde_json::to_string(&results).unwrap();
    return serialized;
}
