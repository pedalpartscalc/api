use super::handlers;
use actix_web::{web, Scope};

pub fn routes() -> Scope {
    web::scope("/parts")
        .service(handlers::get_parts)
        .service(handlers::get_part)
        .service(handlers::new_part)
        .service(handlers::delete_part)
        .service(handlers::update_part)
}
