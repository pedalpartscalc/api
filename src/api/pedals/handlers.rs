use super::types::{RequiredPart, NewRequiredPart, Pedal, PedalPartRow, NewPedal, ClosePedal, AlternatePart};
use crate::api::parts::types::AvailablePart;
use crate::types::Id;
use crate::extractors::Claims;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use sqlx::PgPool;
use itertools::Itertools;
use std::collections::HashSet;

pub fn pedal_rows_to_pedal(rows: &std::vec::Vec<PedalPartRow>) -> Pedal {
    let mut pedal = Pedal {
        id: rows[0].id,
        name: rows[0].name.clone(),
        kind: rows[0].kind.clone(),
        build_doc_link: rows[0].build_doc_link.clone(),
        created_at: rows[0].created_at,
        updated_at: rows[0].updated_at,
        required_parts: vec![],
    };
    for row in rows {
        if row.part_id.is_none() || row.part_name.is_none() || row.part_kind.is_none() || row.part_quantity.is_none() || row.alternate_to.is_some() {
            continue;
        } 
        pedal.required_parts.push(RequiredPart {
            id: row.part_id.unwrap(),
            pedal_id: pedal.id,
            part_name: row.part_name.as_ref().unwrap().clone(),
            part_kind: row.part_kind.as_ref().unwrap().clone(),
            quantity: row.part_quantity.unwrap(),
            alternates: vec![],
        });
    }

    for row in rows {
        if row.alternate_to.is_none() {
            continue;
        }
        let required_part = pedal.required_parts.iter_mut().find(|required_part| required_part.id == row.alternate_to.unwrap());
        if required_part.is_none() {
            // TODO: this should be an error case if we can't find the part
            continue;
        }
        required_part.unwrap().alternates.push(AlternatePart {
            id: row.id,
            pedal_id: pedal.id,
            part_name: row.part_name.as_ref().unwrap().clone(),
            part_kind: row.part_kind.as_ref().unwrap().clone(),
            quantity: row.part_quantity.unwrap(),
        });
    }
    pedal.required_parts.sort_by(|a, b| a.part_kind.to_lowercase().cmp(&b.part_kind.to_lowercase()));
    return pedal;
}

async fn get_pedal_by_id(id: i64, db_pool: web::Data<PgPool>) -> std::io::Result<Pedal> {
    let rows = sqlx::query_as!(
        PedalPartRow,
        r#"SELECT pedals.id, pedals.name, pedals.kind, pedals.build_doc_link, pedals.created_at, pedals.updated_at, required_parts.id AS "part_id?", required_parts.part_name as "part_name?", required_parts.part_kind as "part_kind?", required_parts.quantity as "part_quantity?", required_parts.alternate_to as "alternate_to?"
            FROM pedals 
            LEFT OUTER JOIN required_parts 
                ON pedals.id=required_parts.pedal_id 
            WHERE pedals.id=$1"#,
        id
    )
    .fetch_all(&**db_pool)
    .await
    .expect("Could not fetch pedal rows");
    
    Ok(pedal_rows_to_pedal(&rows))
}

async fn get_all_pedals(db_pool: web::Data<PgPool>) -> std::io::Result<std::vec::Vec<Pedal>> {
    let rows = sqlx::query_as!(
        PedalPartRow,
        r#"SELECT pedals.id, pedals.name, pedals.kind, pedals.build_doc_link, pedals.created_at, pedals.updated_at, required_parts.id AS "part_id?", required_parts.part_name as "part_name?", required_parts.part_kind as "part_kind?", required_parts.quantity as "part_quantity?", required_parts.alternate_to as "alternate_to?"
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
        let pedal_rows = group.into_iter().collect();
        pedals.push(pedal_rows_to_pedal(&pedal_rows));
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

async fn get_owners_parts(owner_id: i64, db_pool: &PgPool) -> std::vec::Vec<AvailablePart> {
    sqlx::query_as!(
        AvailablePart,
        r#"SELECT * FROM available_parts WHERE owner_id=$1"#,
        owner_id
    )
    .fetch_all(db_pool)
    .await
    .expect("Could not fetch parts")
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

    Ok(web::Json(pedals))
}

fn find_closest_pedals(available_parts: &std::vec::Vec<AvailablePart>, pedals: &std::vec::Vec<Pedal>) -> std::vec::Vec<ClosePedal> {
    let mut closest_pedals: std::vec::Vec<ClosePedal> = vec![];
    for p in pedals {
        let mut short_parts = vec![];
        for rp in &p.required_parts {
            let available_part = available_parts.iter().find(|ap| ap.part_name == rp.part_name && ap.part_kind == rp.part_kind);
            if available_part.is_none() {
                short_parts.push(rp.clone());
                continue;
            }
            let available_part = available_part.unwrap();
            if available_part.quantity >= rp.quantity {
                continue;
            }

            short_parts.push(RequiredPart {
                id: rp.id,
                pedal_id: rp.pedal_id,
                part_name: rp.part_name.clone(),
                part_kind: rp.part_kind.clone(),
                quantity: rp.quantity - available_part.quantity,
                alternates: vec![]
            });
        }
        if short_parts.len() > 0 {
            // only add pedals that still need parts
            closest_pedals.push(ClosePedal {
                id: p.id,
                name: p.name.clone(),
                kind: p.kind.clone(),
                short_parts: short_parts,
                required_parts: p.required_parts.clone(),
            });
        }
    }

    closest_pedals.sort_by(|a, b| a.short_parts.len().cmp(&b.short_parts.len()));

    closest_pedals
}

#[get("/closest")]
pub async fn get_closest_available_pedals(claims: Claims, db_pool: web::Data<PgPool>) -> impl Responder {
    let owner_id: i64 = claims.owner_id(&**db_pool).await;
    let available_parts = get_owners_parts(owner_id, &**db_pool).await;

    let pedals = match get_all_pedals(db_pool).await {
        Ok(pedals) => pedals,
        Err(_) => return Err(HttpResponse::NotFound()),
    };

    Ok(web::Json(find_closest_pedals(&available_parts, &pedals)))
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
        r#"INSERT INTO pedals (name, kind, build_doc_link) VALUES ($1, $2, $3) RETURNING id"#,
        new_pedal.name,
        new_pedal.kind,
        new_pedal.build_doc_link
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
        r#"UPDATE pedals SET name=$1, kind=$2, build_doc_link=$3 WHERE id=$4"#,
        update_pedal.name,
        update_pedal.kind,
        update_pedal.build_doc_link,
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
    _claims: Claims,
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
    claims: Claims,
    path: web::Path<Id>,
    required_part: web::Json<NewRequiredPart>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    if !claims.validate_permissions(&HashSet::from(["write:pedals".to_string()])) {
        return Err(HttpResponse::Forbidden());
    }

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
    claims: Claims,
    path: web::Path<(i64, i64)>,
    required_part: web::Json<NewRequiredPart>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    if !claims.validate_permissions(&HashSet::from(["write:pedals".to_string()])) {
        return Err(HttpResponse::Forbidden());
    }

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
    claims: Claims,
    path: web::Path<(i64, i64)>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    if !claims.validate_permissions(&HashSet::from(["write:pedals".to_string()])) {
        return Err(HttpResponse::Forbidden());
    }

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