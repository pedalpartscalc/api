mod types;

use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub fn get_dummy_connection_pool() -> PgPool {
    PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy("postgresql://postgres:postgres@localhost:5432/postgres")
        .expect("Could not connect to database pool")
}

pub fn get_connection_pool(db_url: String) -> PgPool {
    PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy(&db_url)
        .expect("Could not connect to database pool")
}

#[actix_web::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    let config = types::Config::default();
    let dummy_pool = get_dummy_connection_pool();

    // Drop and recreate the database
    // WITH (FORCE) requires postgres > 13
    sqlx::query!(r#"DROP DATABASE pedalpartscalc WITH (FORCE)"#).execute(&dummy_pool).await.expect("Could not drop database");
    sqlx::query!(r#"CREATE DATABASE pedalpartscalc"#).execute(&dummy_pool).await.expect("Could not drop database");

    let db_pool = get_connection_pool(config.database_url);

    sqlx::migrate!().run(&db_pool).await.expect("Could not migrate database");
    
    let pedal_id = sqlx::query_as!(
        types::Id,
        r#"INSERT INTO pedals (name, kind) VALUES ($1, $2) RETURNING id"#,
        "Test Pedal",
        "Overdrive"
    )
    .fetch_one(&db_pool)
    .await
    .expect("Could not insert pedal");

    sqlx::query!(
        r#"INSERT INTO required_parts (pedal_id, part_name, part_kind, quantity) VALUES ($1, $2, $3, $4)"#,
        pedal_id.id,
        "2n5908",
        "Transistor",
        3
    )
    .execute(&db_pool)
    .await
    .expect("Could not insert part");
    
    sqlx::query!(
        r#"INSERT INTO required_parts (pedal_id, part_name, part_kind, quantity) VALUES ($1, $2, $3, $4)"#,
        pedal_id.id,
        "1n4148",
        "Diode",
        2
    )
    .execute(&db_pool)
    .await
    .expect("Could not insert part");

    println!("Seeded database!");
}