#[macro_use]
extern crate rocket;

mod parts;

#[get("/")]
fn index() -> String {
    format!("Parts {:?}", parts::create_example_parts_list())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
