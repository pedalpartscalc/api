use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use pedalpartscalc::types::Id;
use pedalpartscalc::api::pedals::handlers::{remove_unavailable_pedals, pedal_rows_to_pedal};
use pedalpartscalc::api::pedals::types::{PedalPartRow, Pedal};
use pedalpartscalc::api::parts::types::{AvailablePart};
use itertools::Itertools;

pub fn get_dummy_connection_pool() -> PgPool {
    PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy("postgresql://postgres:postgres@localhost:5432/postgres")
        .expect("Could not connect to database pool")
}

pub fn get_connection_pool() -> PgPool {
    PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy("postgresql://postgres:postgres@localhost:5432/pedalpartscalc_test")
        .expect("Could not connect to database pool")
}

async fn set_up_database() -> PgPool {
    // drop and recreate the database
    let dummy_pool = get_dummy_connection_pool();
    match sqlx::query!(r#"DROP DATABASE pedalpartscalc_test WITH (FORCE)"#).execute(&dummy_pool).await {
        Ok(_) => (),
        Err(e) => {
            println!("{}", e);
        }
    };
    sqlx::query!(r#"CREATE DATABASE pedalpartscalc_test"#).execute(&dummy_pool).await.expect("Could not drop database");

    let db_pool = get_connection_pool();
    sqlx::migrate!().run(&db_pool).await.expect("Could not migrate database");

    db_pool
}

// Keeping this around as a reminder to never try running a generic query in a function
// async fn create_pedal<'e, Exe>(pool: Exe) -> <<Exe as sqlx::Executor<'e>>::Database as sqlx::Database>::QueryResult
// where
//     Exe: sqlx::Executor<'e>,
//     <Exe::Database as sqlx::database::HasArguments<'e>>::Arguments:
//         sqlx::IntoArguments<'e, <Exe as sqlx::Executor<'e>>::Database>,
// {
//     match sqlx::query(
//         r#"INSERT INTO pedals (name, kind) VALUES ('Test Pedal', 'Overdrive') RETURNING id"#,
//     )
//     .execute(pool)
//     .await {
//         Ok(id) => id,
//         Err(e) => panic!("Could not insert pedal: {}", e),
//     }
// }

#[actix_rt::test]
async fn test_available_pedals() {
    let db_pool = set_up_database().await;
    let mut tx = db_pool.begin().await.expect("Could not start transaction");

    // Create two pedals
    let first_pedal_id = sqlx::query_as!(
        Id,
        r#"INSERT INTO pedals (name, kind) VALUES ($1, $2) RETURNING id"#,
        "Pedal 1",
        "Overdrive"
    )
    .fetch_one(&mut tx)
    .await
    .expect("Could not insert pedal");
    let second_pedal_id = sqlx::query_as!(
        Id,
        r#"INSERT INTO pedals (name, kind) VALUES ($1, $2) RETURNING id"#,
        "Pedal 2",
        "Delay"
    )
    .fetch_one(&mut tx)
    .await
    .expect("Could not insert pedal");

    // Create required parts for pedal 1
    sqlx::query!(
        r#"INSERT INTO required_parts (pedal_id, part_name, part_kind, quantity) VALUES ($1, $2, $3, $4)"#,
        first_pedal_id.id,
        "Part 1",
        "Transistor",
        2
    )
    .execute(&mut tx)
    .await
    .expect("Could not insert part");
    sqlx::query!(
        r#"INSERT INTO required_parts (pedal_id, part_name, part_kind, quantity) VALUES ($1, $2, $3, $4)"#,
        first_pedal_id.id,
        "Part 2",
        "Transistor",
        2
    )
    .execute(&mut tx)
    .await
    .expect("Could not insert part");

    // Create required parts for pedal 2
    // Pedal 2 needs **3** of Part 2 instead of 2
    sqlx::query!(
        r#"INSERT INTO required_parts (pedal_id, part_name, part_kind, quantity) VALUES ($1, $2, $3, $4)"#,
        second_pedal_id.id,
        "Part 1",
        "Transistor",
        2
    )
    .execute(&db_pool)
    .await
    .expect("Could not insert part");
    sqlx::query!(
        r#"INSERT INTO required_parts (pedal_id, part_name, part_kind, quantity) VALUES ($1, $2, $3, $4)"#,
        second_pedal_id.id,
        "Part 2",
        "Transistor",
        3
    )
    .execute(&mut tx)
    .await
    .expect("Could not insert part");

    // Create 2 avaialable parts of each type
    sqlx::query!(
        r#"INSERT INTO available_parts (owner_id, part_name, part_kind, quantity) VALUES ($1, $2, $3, $4)"#,
        1,
        "Part 1",
        "Transistor",
        2
    )
    .execute(&mut tx)
    .await
    .expect("Could not insert part");
    sqlx::query!(
        r#"INSERT INTO available_parts (owner_id, part_name, part_kind, quantity) VALUES ($1, $2, $3, $4)"#,
        1,
        "Part 2",
        "Transistor",
        2
    )
    .execute(&mut tx)
    .await
    .expect("Could not insert part");

    // Get the pedals and parts lists
    let rows = sqlx::query_as!(
        PedalPartRow,
        r#"SELECT pedals.id, pedals.name, pedals.kind, pedals.created_at, pedals.updated_at, required_parts.id AS "part_id?", required_parts.part_name as "part_name?", required_parts.part_kind as "part_kind?", required_parts.quantity as "part_quantity?"
            FROM pedals 
            LEFT OUTER JOIN required_parts 
                ON pedals.id=required_parts.pedal_id
        ORDER BY pedals.id"#,
    )
    .fetch_all(&mut tx)
    .await
    .expect("Could not fetch pedal rows");

    let mut pedals: std::vec::Vec<Pedal> = vec![];
    for (_key, group) in &rows.into_iter().group_by(|row| row.id) {
        pedals.push(pedal_rows_to_pedal(group.into_iter().collect()));
    }

    let available_parts = sqlx::query_as!(
        AvailablePart,
        r#"SELECT * FROM available_parts WHERE owner_id=$1"#,
        1
    )
    .fetch_all(&mut tx)
    .await
    .expect("Could not fetch parts");


    remove_unavailable_pedals(&mut pedals, available_parts);
    assert_eq!(pedals.len(), 1);
}