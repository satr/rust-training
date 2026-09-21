use sqlx::postgres::PgPool;

#[derive(Debug, PartialEq, sqlx::FromRow)]
struct Reading {
    device_id: String,
    value: i32,
}

const CREATE_READINGS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS readings (
    device_id TEXT NOT NULL,
    value INTEGER NOT NULL
)
"#;

async fn ensure_schema(pool: &PgPool) -> Result<(), sqlx::Error> {
    todo!()
}

async fn insert_reading(pool: &PgPool, reading: &Reading) -> Result<(), sqlx::Error> {
    todo!()
}

async fn list_readings(pool: &PgPool) -> Result<Vec<Reading>, sqlx::Error> {
    todo!()
}

#[cfg(test)]
#[path = "../../test_support/postgres.rs"]
mod postgres;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("PostgreSQL repository integration exercise");
}

#[cfg(test)]
mod tests {
    use super::{ensure_schema, insert_reading, list_readings, postgres, Reading, CREATE_READINGS_TABLE};

    #[test]
    fn schema_has_required_columns() {
        assert!(CREATE_READINGS_TABLE.contains("device_id TEXT NOT NULL"));
        assert!(CREATE_READINGS_TABLE.contains("value INTEGER NOT NULL"));
    }

    #[tokio::test(flavor = "current_thread")]
    #[ignore = "requires DATABASE_URL and a running PostgreSQL instance"]
    async fn repository_round_trip() {
        let pool = postgres::postgres_pool().await;

        ensure_schema(&pool).await.unwrap();
        sqlx::query("TRUNCATE readings")
            .execute(&pool)
            .await
            .unwrap();

        let expected = Reading {
            device_id: "sensor-1".to_owned(),
            value: 42,
        };
        insert_reading(&pool, &expected).await.unwrap();

        assert_eq!(list_readings(&pool).await.unwrap(), vec![expected]);
    }
}
