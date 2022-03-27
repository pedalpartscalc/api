use super::types::{AvailablePart, NewAvailablePart, PartId};
use crate::extractors::Claims;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use sqlx::PgPool;

#[get("")]
pub async fn get_parts(claims: Claims, db_pool: web::Data<PgPool>) -> impl Responder {
    let owner_id: i64 = claims.owner_id(&**db_pool).await;
    let mut parts = match sqlx::query_as!(
        AvailablePart,
        r#"SELECT * FROM available_parts WHERE owner_id=$1"#,
        owner_id
    )
    .fetch_all(&**db_pool)
    .await
    {
        Ok(parts) => parts,
        Err(_) => return Err(HttpResponse::Forbidden()),
    };
    parts.sort_by(|a, b| a.part_kind.to_lowercase().cmp(&b.part_kind.to_lowercase()));
    Ok(web::Json(parts))
}

#[get("/{id}")]
pub async fn get_part(
    claims: Claims,
    path: web::Path<PartId>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let owner_id: i64 = claims.owner_id(&**db_pool).await;
    let part = sqlx::query_as!(
        AvailablePart,
        r#"SELECT * FROM available_parts WHERE id=$1"#,
        path.id
    )
    .fetch_one(&**db_pool)
    .await
    .expect("Could not fetch part");
    if part.owner_id != owner_id {
        return Err(HttpResponse::Forbidden());
    }

    Ok(web::Json(part))
}

#[post("")]
pub async fn new_part(
    claims: Claims,
    part: web::Json<NewAvailablePart>,
    db_pool: web::Data<PgPool>,
) -> HttpResponse {
    let owner_id: i64 = claims.owner_id(&**db_pool).await;
    let part = part.into_inner();
    let part_id = sqlx::query_as!(
        PartId,
        r#"INSERT INTO available_parts (owner_id, part_name, part_kind, quantity) VALUES ($1, $2, $3, $4) RETURNING id"#,
        owner_id,
        &part.part_name,
        &part.part_kind,
        &part.quantity
    )
    .fetch_one(&**db_pool)
    .await
    .expect("Could not insert part");

    HttpResponse::Ok().json(part_id.id)
}

#[delete("/{id}")]
pub async fn delete_part(claims: Claims, path: web::Path<PartId>, db_pool: web::Data<PgPool>) -> HttpResponse {
    let owner_id: i64 = claims.owner_id(&**db_pool).await;
    sqlx::query!(r#"DELETE FROM available_parts WHERE id=$1 AND owner_id=$2"#, path.id, owner_id)
        .execute(&**db_pool)
        .await
        .expect("Could not delete part");
    HttpResponse::Ok().finish()
}

#[put("/{id}")]
pub async fn update_part(
    claims: Claims,
    path: web::Path<PartId>,
    part: web::Json<NewAvailablePart>,
    db_pool: web::Data<PgPool>,
) -> HttpResponse {
    let owner_id: i64 = claims.owner_id(&**db_pool).await;
    let part = part.into_inner();
    sqlx::query!(
        r#"UPDATE available_parts SET part_name=$1, part_kind=$2, quantity=$3 WHERE id=$4 and owner_id=$5"#,
        &part.part_name,
        &part.part_kind,
        &part.quantity,
        path.id,
        owner_id
    )
    .execute(&**db_pool)
    .await
    .expect("Could not update part");
    HttpResponse::Ok().finish()
}
