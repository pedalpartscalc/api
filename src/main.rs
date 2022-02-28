mod api;
mod extractors;
mod middlewares;
mod types;

use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub fn get_connection_pool(db_url: String) -> PgPool {
    PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy(&db_url)
        .expect("Could not connect to database pool")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let config = types::Config::default();
    println!("{}", &config.client_origin_url);
    let auth0_config = extractors::Auth0Config::default();
    let db_pool = Data::new(get_connection_pool(config.database_url));
    HttpServer::new(move || {
        App::new()
            .app_data(auth0_config.clone())
            .wrap(middlewares::cors(&config.client_origin_url))
            .wrap(middlewares::err_handlers())
            .wrap(middlewares::security_headers())
            .wrap(middlewares::logger())
            .service(api::routes())
            .app_data(db_pool.clone())
    })
    .bind(("127.0.0.1", config.port))?
    .run()
    .await
}
