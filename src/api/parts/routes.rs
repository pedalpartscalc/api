use super::handlers;
use actix_web::{web, Scope};

pub fn routes() -> Scope {
    web::scope("/parts").service(handlers::get_part)
}
