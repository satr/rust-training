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

async fn insert_readings_transactionally(
    pool: &PgPool,
    readings: &[Reading],
) -> Result<(), sqlx::Error> {
    todo!()
}

#[cfg(test)]
#[path = "../../test_support/postgres.rs"]
mod postgres;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("PostgreSQL transaction exercise");
}

#[cfg(test)]
mod tests {
    use super::{insert_readings_transactionally, postgres, Reading, CREATE_READINGS_TABLE};

    #[test]
    fn schema_is_available_for_transaction_tests() {
        assert!(CREATE_READINGS_TABLE.contains("CREATE TABLE IF NOT EXISTS readings"));
    }

    #[tokio::test(flavor = "current_thread")]
    #[ignore = "requires DATABASE_URL and a running PostgreSQL instance"]
    async fn inserts_the_whole_batch() {
        let pool = postgres::postgres_pool().await;
        sqlx::query(CREATE_READINGS_TABLE)
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("TRUNCATE readings")
            .execute(&pool)
            .await
            .unwrap();

        let readings = vec![
            Reading { device_id: "a".to_owned(), value: 1 },
            Reading { device_id: "b".to_owned(), value: 2 },
        ];

        insert_readings_transactionally(&pool, &readings)
            .await
            .unwrap();

        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM readings")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(count.0, 2);
    }
}
