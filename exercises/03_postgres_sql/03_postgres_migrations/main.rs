use sqlx::postgres::PgPool;

const CREATE_READINGS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS readings (
    device_id TEXT NOT NULL,
    value INTEGER NOT NULL
)
"#;

async fn ensure_schema(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(CREATE_READINGS_TABLE).execute(pool).await?;
    Ok(())
}

#[cfg(test)]
#[path = "../../test_support/postgres.rs"]
mod postgres;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("{}", CREATE_READINGS_TABLE.trim());
}

#[cfg(test)]
mod tests {
    use super::{ensure_schema, postgres, CREATE_READINGS_TABLE};

    #[test]
    fn migration_creates_expected_table() {
        assert!(CREATE_READINGS_TABLE.contains("CREATE TABLE IF NOT EXISTS readings"));
        assert!(CREATE_READINGS_TABLE.contains("device_id TEXT NOT NULL"));
        assert!(CREATE_READINGS_TABLE.contains("value INTEGER NOT NULL"));
    }

    #[tokio::test(flavor = "current_thread")]
    #[ignore = "requires DATABASE_URL and a running PostgreSQL instance"]
    async fn applies_schema_to_postgres() {
        let pool = postgres::postgres_pool().await;

        ensure_schema(&pool).await.unwrap();
    }
}
