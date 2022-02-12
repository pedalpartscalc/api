#[macro_use]
extern crate rocket;
extern crate hello_rocket;

use self::hello_rocket::*;
use rocket::response::status;
use rocket::serde::json::Json;

#[get("/parts")]
fn get_parts() -> String {
    views::list_parts()
}

#[post("/parts", data = "<task>")]
fn new_part(task: Json<models::NewAvailablePart>) -> status::Accepted<String> {
    println!("{:?}", task);
    let connection = db::establish_connection();
    views::create_available_part(&connection, &task.part_name, &task.part_kind, task.quantity);
    return status::Accepted(Some("".to_string()));
}

#[get("/")]
fn index() -> String {
    format!("Parts {:?}", parts::create_example_parts_list())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_parts, new_part])
}
