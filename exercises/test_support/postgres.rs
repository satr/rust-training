use sqlx::postgres::{PgPool, PgPoolOptions};

pub async fn postgres_pool() -> PgPool {
    let database_url = std::env::var("DATABASE_URL").unwrap();

    PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await
        .unwrap()
}
