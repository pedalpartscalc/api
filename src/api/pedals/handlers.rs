use super::types::{RequiredPart, NewRequiredPart, Pedal, PedalPartRow, NewPedal};
use crate::api::parts::types::AvailablePart;
use crate::types::Id;
use crate::extractors::Claims;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use sqlx::PgPool;
use itertools::Itertools;

// TODO: Make all of these hidden behind Admin access

pub fn pedal_rows_to_pedal(rows: std::vec::Vec<PedalPartRow>) -> Pedal {
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
    for (_, group) in &rows.into_iter().group_by(|row| row.id) {
        pedals.push(pedal_rows_to_pedal(group.into_iter().collect()));
    }
    
    Ok(pedals)
}

#[get("")]
pub async fn get_pedals(db_pool: web::Data<PgPool>) -> impl Responder {
    match get_all_pedals(db_pool).await {
        Ok(pedals) => Ok(web::Json(pedals)),
        Err(err) => Err(HttpResponse::InternalServerError().json(err.to_string())),
    }
}

#[get("/{id}")]
pub async fn get_pedal(path: web::Path<Id>, _claims: Claims, db_pool: web::Data<PgPool>) -> impl Responder {
    match get_pedal_by_id(path.id, db_pool).await {
        Ok(pedal) => Ok(web::Json(pedal)),
        Err(_) => Err(HttpResponse::NotFound()),
    }
}

pub fn remove_unavailable_pedals(pedals: &mut std::vec::Vec<Pedal>, available_parts: std::vec::Vec<AvailablePart>) -> () {
    pedals.retain(|pedal| {
        pedal.required_parts.iter().all(|part| {
            available_parts.iter().find(|available_part| available_part.part_name == part.part_name && available_part.part_kind == part.part_kind && available_part.quantity >= part.quantity).is_some()
        })
    });
}

#[get("/available")]
pub async fn get_available_pedals(claims: Claims, db_pool: web::Data<PgPool>) -> impl Responder {
    let owner_id: i64 = claims.owner_id(&**db_pool).await;
    let available_parts = sqlx::query_as!(
        AvailablePart,
        r#"SELECT * FROM available_parts WHERE owner_id=$1"#,
        owner_id
    )
    .fetch_all(&**db_pool)
    .await
    .expect("Could not fetch parts");

    let mut pedals = match get_all_pedals(db_pool).await {
        Ok(pedals) => pedals,
        Err(_) => return Err(HttpResponse::NotFound()),
    };

    remove_unavailable_pedals(&mut pedals, available_parts);

    return Ok(web::Json(pedals));
}

#[post("")]
pub async fn new_pedal(
    _claims: Claims,
    new_pedal: web::Json<NewPedal>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let new_pedal = new_pedal.into_inner();
    match sqlx::query_as!(
        Id,
        r#"INSERT INTO pedals (name, kind) VALUES ($1, $2) RETURNING id"#,
        new_pedal.name,
        new_pedal.kind
    )
    .fetch_one(&**db_pool)
    .await {
        Ok(id) => Ok(web::Json(id.id)),
        Err(_) => Err(HttpResponse::InternalServerError()),
    }
}

#[put("/{id}")]
pub async fn update_pedal(
    _claims: Claims,
    path: web::Path<Id>,
    pedal: web::Json<NewPedal>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let update_pedal = pedal.into_inner();
    match sqlx::query!(
        r#"UPDATE pedals SET name=$1, kind=$2 WHERE id=$3"#,
        update_pedal.name,
        update_pedal.kind,
        path.id
    )
    .execute(&**db_pool)
    .await {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Err(HttpResponse::InternalServerError()),
    }
}

#[delete("/{id}")]
pub async fn delete_pedal(
    _claimms: Claims,
    path: web::Path<Id>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    match sqlx::query!(
        r#"DELETE FROM pedals WHERE id=$1"#,
        path.id
    )
    .execute(&**db_pool)
    .await {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Err(HttpResponse::InternalServerError()),
    }
}

#[post("/{id}/parts")]
pub async fn create_required_part(
    _claims: Claims,
    path: web::Path<Id>,
    required_part: web::Json<NewRequiredPart>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let required_part = required_part.into_inner();
    match sqlx::query_as!(
        Id,
        r#"INSERT INTO required_parts (pedal_id, part_name, part_kind, quantity) VALUES ($1, $2, $3, $4) RETURNING id"#,
        path.id,
        required_part.part_name,
        required_part.part_kind,
        required_part.quantity
    )
    .fetch_one(&**db_pool)
    .await {
        Ok(id) => Ok(web::Json(id.id)),
        Err(_) => Err(HttpResponse::InternalServerError()),
    }
}

#[put("/{id}/parts/{part_id}")]
pub async fn update_required_part(
    _claims: Claims,
    path: web::Path<(i64, i64)>,
    required_part: web::Json<NewRequiredPart>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let (pedal_id, part_id) = path.into_inner();
    let required_part = required_part.into_inner();
    match sqlx::query!(
        r#"UPDATE required_parts SET part_name=$1, part_kind=$2, quantity=$3 WHERE pedal_id=$4 AND id=$5"#,
        required_part.part_name,
        required_part.part_kind,
        required_part.quantity,
        pedal_id,
        part_id
    )
    .execute(&**db_pool)
    .await {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Err(HttpResponse::InternalServerError()),
    }
}

#[delete("/{id}/parts/{part_id}")]
pub async fn delete_required_part(
    _claims: Claims,
    path: web::Path<(i64, i64)>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let (pedal_id, part_id) = path.into_inner();
    match sqlx::query!(
        r#"DELETE FROM required_parts WHERE pedal_id=$1 AND id=$2"#,
        pedal_id,
        part_id
    )
    .execute(&**db_pool)
    .await {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Err(HttpResponse::InternalServerError()),
    }
}