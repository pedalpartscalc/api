#[macro_use]
extern crate rocket;
extern crate hello_rocket;

use self::hello_rocket::*;
use crate::schema::available_parts;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use rocket::response::status;
use rocket::serde::json::Json;

#[get("/parts")]
fn get_parts() -> String {
    views::list_parts()
}

#[post("/parts", data = "<task>")]
fn new_part(task: Json<models::NewAvailablePart>) -> status::Accepted<String> {
    let connection = db::establish_connection();
    views::create_available_part(&connection, &task.part_name, &task.part_kind, task.quantity);
    return status::Accepted(Some("".to_string()));
}

#[delete("/parts/<pk>")]
fn delete_part(pk: i64) -> status::Accepted<String> {
    let connection = db::establish_connection();

    // Had weird error related to the fact I was trying to use an i32 for the primary key and it's an i64 in schema
    diesel::delete(available_parts::table.find(pk))
        .execute(&connection)
        .expect("Error deleting Part");
    return status::Accepted(Some("".to_string()));
}

#[put("/<pk>", format = "application/json", data = "<part>")]
pub fn update_post(pk: i64, part: Json<models::AvailablePart>) -> status::Accepted<String> {
    let connection = db::establish_connection();
    diesel::update(available_parts::table.find(pk))
        .set(&*part)
        .execute(&connection)
        .expect("Error updating Part");
    return status::Accepted(Some("".to_string()));
}

#[get("/")]
fn index() -> String {
    format!("Parts {:?}", parts::create_example_parts_list())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_parts, new_part, delete_part])
}
