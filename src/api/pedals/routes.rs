use super::handlers;
use actix_web::{web, Scope};

pub fn routes() -> Scope {
    web::scope("/pedals")
        .service(handlers::get_available_pedals)
        .service(handlers::get_closest_available_pedals)
        .service(handlers::get_pedal)
        .service(handlers::get_pedals)
        .service(handlers::new_pedal)
        .service(handlers::update_pedal)
        .service(handlers::delete_pedal)
        .service(handlers::create_required_part)
        .service(handlers::update_required_part)
        .service(handlers::delete_required_part)
}
