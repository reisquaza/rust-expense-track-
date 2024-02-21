use sqlx::postgres::PgPoolOptions;

async fn postgres_pool(url: &str) -> Result<sqlx::PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(url)
        .await?;
    Ok(pool)
}