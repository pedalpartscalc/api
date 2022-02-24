use super::messages;
use super::parts;
use actix_web::{web, Scope};

pub fn routes() -> Scope {
    web::scope("/api")
        .service(messages::routes())
        .service(parts::routes())
}
