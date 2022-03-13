use super::handlers;
use actix_web::{web, Scope};

pub fn routes() -> Scope {
    web::scope("/pedals")
        .service(handlers::get_available_pedals)
        .service(handlers::get_pedal)
        .service(handlers::get_pedals)
        .service(handlers::new_pedal)
        .service(handlers::update_pedal)
}
