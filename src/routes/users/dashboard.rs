// use crate::session_state::TypedSession;
// use crate::utils::e500;
// use actix_web::http::header::{ContentType, LOCATION};
// use actix_web::{web, HttpResponse};
// use anyhow::Context;
// use sqlx::PgPool;

// /// This seems to be the general idea behind session auth
// pub async fn admin_dashboard(
//     session: TypedSession,
//     pool: web::Data<PgPool>,
// ) -> Result<HttpResponse, actix_web::Error> {
//     let username = if let Some(user_id) = session.get_user_id().map_err(e500)? {
//         get_username(user_id, &pool).await.map_err(e500)?
//     } else {
//         return Ok(HttpResponse::SeeOther()
//             .insert_header((LOCATION, "/login"))
//             .finish());
//     };
//     Ok(HttpResponse::SeeOther()
//         .insert_header((LOCATION, "/login"))
//         .finish())
// }

// // pub async fn example(session: TypedSession,) -> Result<HttpResponse, actix_web::Error> {
// //     if session.get_user_id().map_err(e500)?.is_none() {
// //         return Ok(see_other("/login"));
// //     };
// // }

// #[tracing::instrument(name = "Get username", skip(pool))]
// pub async fn get_username(user_id: uuid::Uuid, pool: &PgPool) -> Result<String, anyhow::Error> {
//     let row = sqlx::query!(
//         r#"
//         SELECT username
//         FROM users
//         WHERE id = $1
//         "#,
//         user_id,
//     )
//     .fetch_one(pool)
//     .await
//     .context("Failed to performed a query to retrieve a username.")?;
//     Ok(row.username)
// }
