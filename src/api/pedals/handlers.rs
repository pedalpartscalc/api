use super::types::{RequiredPart, NewRequiredPart, Pedal, PedalPartRow, NewPedal};
use crate::api::parts::types::AvailablePart;
use crate::types::Id;
use crate::extractors::Claims;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use sqlx::PgPool;
use itertools::Itertools;

// TODO: Make all of these hidden behind Admin access

fn pedal_rows_to_pedal(rows: std::vec::Vec<PedalPartRow>) -> Pedal {
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
    return pedal;
}

async fn get_pedal_by_id(id: i64, db_pool: web::Data<PgPool>) -> std::io::Result<Pedal> {
    let rows = sqlx::query_as!(
        PedalPartRow,
        r#"SELECT pedals.id, pedals.name, pedals.kind, pedals.created_at, pedals.updated_at, required_parts.id AS "part_id?", required_parts.part_name as "part_name?", required_parts.part_kind as "part_kind?", required_parts.quantity as "part_quantity?"
            FROM pedals 
            LEFT OUTER JOIN required_parts 
                ON pedals.id=required_parts.pedal_id 
            WHERE pedals.id=$1"#,
        id
    )
    .fetch_all(&**db_pool)
    .await
    .expect("Could not fetch pedal rows");
    
    Ok(pedal_rows_to_pedal(rows))
}

async fn get_all_pedals(db_pool: web::Data<PgPool>) -> std::io::Result<std::vec::Vec<Pedal>> {
    let rows = sqlx::query_as!(
        PedalPartRow,
        r#"SELECT pedals.id, pedals.name, pedals.kind, pedals.created_at, pedals.updated_at, required_parts.id AS "part_id?", required_parts.part_name as "part_name?", required_parts.part_kind as "part_kind?", required_parts.quantity as "part_quantity?"
            FROM pedals 
            LEFT OUTER JOIN required_parts 
                ON pedals.id=required_parts.pedal_id
        ORDER BY pedals.id"#,
    )
    .fetch_all(&**db_pool)
    .await
    .expect("Could not fetch pedal rows");

    let mut pedals: std::vec::Vec<Pedal> = vec![];
    for (key, group) in &rows.into_iter().group_by(|row| row.id) {
        println!("{}", key);
        pedals.push(pedal_rows_to_pedal(group.into_iter().collect()));
    }
    
    Ok(pedals)
}

#[get("/{id}")]
pub async fn get_pedal(path: web::Path<Id>, _claims: Claims, db_pool: web::Data<PgPool>) -> impl Responder {
    match get_pedal_by_id(path.id, db_pool).await {
        Ok(pedal) => Ok(web::Json(pedal)),
        Err(_) => Err(HttpResponse::NotFound()),
    }
}

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

//     Ok(web::Json({}))
// }

#[get("/available")]
pub async fn get_available_pedals(_claims: Claims, db_pool: web::Data<PgPool>) -> impl Responder {
    match get_all_pedals(db_pool).await {
        Ok(pedals) => Ok(web::Json(pedals)),
        Err(_) => Err(HttpResponse::NotFound()),
    }
}