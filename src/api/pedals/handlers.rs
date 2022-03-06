use super::types::{RequiredPart, NewRequiredPart, Pedal, PedalPartRow, NewPedal};
use crate::api::parts::types::AvailablePart;
use crate::types::Id;
use crate::extractors::Claims;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use sqlx::PgPool;

// TODO: Make all of these hidden behind Admin access

// #[get("/available")]
// pub async fn get_available_pedals(claims: Claims, db_pool: web::Data<PgPool>) -> impl Responder {
//     let owner_id: i64 = claims.owner_id(&**db_pool).await;
//     let available_parts = sqlx::query_as!(
//         AvailablePart,
//         r#"SELECT * FROM available_parts WHERE owner_id=$1"#,
//         owner_id
//     )
//     .fetch_all(&**db_pool)
//     .await
//     .expect("Could not fetch parts");

//     Ok(HttpResponse::Ok().json(available_parts))
//     // Get all of the parts for a given pedal
// }

#[get("/{id}")]
pub async fn get_pedal(path: web::Path<Id>, _claims: Claims, db_pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query_as!(
        PedalPartRow,
        r#"SELECT pedals.id, pedals.name, pedals.kind, pedals.created_at, pedals.updated_at, required_parts.id AS "part_id?", required_parts.part_name as "part_name?", required_parts.part_kind as "part_kind?", required_parts.quantity as "part_quantity?"
            FROM pedals 
            LEFT OUTER JOIN required_parts 
                ON pedals.id=required_parts.pedal_id 
            WHERE pedals.id=$1"#,
        path.id
    )
    .fetch_all(&**db_pool)
    .await
    {
        Ok(rows) => {
            println!("{:?}", rows);
            let mut pedal = Pedal {
                id: rows[0].id,
                name: rows[0].name.clone(),
                kind: rows[0].kind.clone(),
                created_at: rows[0].created_at,
                updated_at: rows[0].updated_at,
                required_parts: vec![],
            };
            for row in rows {
                if row.part_id.is_none() || row.part_name.is_none() || row.part_kind.is_none() || row.part_quantity.is_none() {
                    continue;
                } 
                pedal.required_parts.push(RequiredPart {
                    id: row.part_id.unwrap(),
                    pedal_id: pedal.id,
                    part_name: row.part_name.unwrap().clone(),
                    part_kind: row.part_kind.unwrap().clone(),
                    quantity: row.part_quantity.unwrap(),
                });
            }
            Ok(web::Json(pedal))
        },
        Err(e) => {
            println!("{:?}", e);
            Err(HttpResponse::NotFound())
        },
    }
}