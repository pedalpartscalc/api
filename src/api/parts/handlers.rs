use super::types::{AvailablePart, PartId};
use actix_web::{get, web};
use sqlx::PgPool;

// #[get("/")]
// pub async fn get_parts() -> impl Responder {
//     todo!()
// }

#[get("/{id}")]
async fn get_part(
    path: web::Query<PartId>,
    db_pool: web::Data<PgPool>,
) -> std::io::Result<web::Json<AvailablePart>> {
    let part = sqlx::query_as!(
        AvailablePart,
        r#"SELECT * FROM available_parts WHERE id=$1"#,
        path.id
    )
    .fetch_one(&**db_pool)
    .await
    .expect("Could not fetch part");

    Ok(web::Json(part))
    // Ok(web::Json({}))
    // let serialized = serde_json::to_string(&part).unwrap();
    // return serialized;
}

// #[post("/parts", data = "<task>")]
// fn new_part(task: Json<models::NewAvailablePart>) -> status::Accepted<String> {
//     let connection = db::establish_connection();
//     views::create_available_part(&connection, &task.part_name, &task.part_kind, task.quantity);
//     return status::Accepted(Some("".to_string()));
// }

// #[delete("/parts/<pk>")]
// fn delete_part(pk: i64) -> status::Accepted<String> {
//     let connection = db::establish_connection();

//     // Had weird error related to the fact I was trying to use an i32 for the primary key and it's an i64 in schema
//     diesel::delete(available_parts::table.find(pk))
//         .execute(&connection)
//         .expect("Error deleting Part");
//     return status::Accepted(Some("".to_string()));
// }

// #[put("/<pk>", format = "application/json", data = "<part>")]
// pub fn update_post(pk: i64, part: Json<models::AvailablePart>) -> status::Accepted<String> {
//     let connection = db::establish_connection();
//     diesel::update(available_parts::table.find(pk))
//         .set(&*part)
//         .execute(&connection)
//         .expect("Error updating Part");
//     return status::Accepted(Some("".to_string()));
// }
