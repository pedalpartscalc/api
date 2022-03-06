use super::{messages, parts, pedals};
use actix_web::{web, Scope};

pub fn routes() -> Scope {
    web::scope("/api")
        .service(messages::routes())
        .service(parts::routes())
        .service(pedals::routes())
}
