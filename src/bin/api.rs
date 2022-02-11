#[macro_use]
extern crate rocket;
extern crate hello_rocket;

use self::hello_rocket::*;

#[get("/parts")]
fn get_parts() -> String {
    views::list_parts()
}

#[get("/")]
fn index() -> String {
    format!("Parts {:?}", parts::create_example_parts_list())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_parts])
}
