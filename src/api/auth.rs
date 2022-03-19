use crate::extractors::Claims;
use actix_web::{post, web, Responder, HttpResponse, Scope};
use std::collections::HashSet;

#[post("/is_admin")]
async fn is_admin(claims: Claims) -> impl Responder {
    match claims.validate_permissions(&HashSet::from(["write:pedals".to_string()])) {
        true => Ok(web::Json(true)),
        false => Err(HttpResponse::Forbidden())
    }
}

pub fn routes() -> Scope {
    web::scope("/auth")
        .service(is_admin)
}