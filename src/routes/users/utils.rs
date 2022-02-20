use anyhow::Context;
use sqlx::PgPool;

#[tracing::instrument(name = "Get username", skip(pool))]
pub async fn get_username(user_id: i64, pool: &PgPool) -> Result<String, anyhow::Error> {
    let row = sqlx::query!(
        r#"
        SELECT username
        FROM users
        WHERE id = $1
        "#,
        user_id,
    )
    .fetch_one(pool)
    .await
    .context("Failed to performed a query to retrieve a username.")?;
    Ok(row.username)
}
