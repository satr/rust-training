use sqlx::postgres::PgPool;

#[derive(Debug, PartialEq, sqlx::FromRow)]
struct Reading {
    device_id: String,
    value: i32,
}

const CREATE_READINGS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS readings (
    device_id TEXT NOT NULL,
    value INTEGER NOT NULL CHECK (value >= 0)
)
"#;

const INSERT_READING_COMMAND: &str = r#"
INSERT INTO readings (device_id, value) VALUES ($1, $2)
"#;

async fn insert_readings_transactionally(
    pool: &PgPool,
    readings: &[Reading],
) -> Result<(), sqlx::Error> {
    // Keep the transaction alive until every insert succeeds.
    todo!()
}

#[cfg(test)]
#[path = "../../test_support/postgres.rs"]
mod postgres;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("PostgreSQL rollback exercise");
}

#[cfg(test)]
mod tests {
    use super::{insert_readings_transactionally, postgres, Reading, CREATE_READINGS_TABLE};
    use std::sync::{Mutex, OnceLock};

    fn database_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    #[test]
    fn schema_rejects_negative_values() {
        assert!(CREATE_READINGS_TABLE.contains("CHECK (value >= 0)"));
    }

    #[tokio::test(flavor = "current_thread")]
    #[ignore = "requires DATABASE_URL and a running PostgreSQL instance"]
    async fn commits_a_valid_batch() {
        let _lock = database_lock().lock().unwrap();
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
            Reading {
                device_id: "a".to_owned(),
                value: 1,
            },
            Reading {
                device_id: "b".to_owned(),
                value: 2,
            },
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

    #[tokio::test(flavor = "current_thread")]
    #[ignore = "requires DATABASE_URL and a running PostgreSQL instance"]
    async fn rolls_back_the_whole_batch_when_one_insert_fails() {
        let _lock = database_lock().lock().unwrap();
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
            Reading {
                device_id: "before-error".to_owned(),
                value: 1,
            },
            Reading {
                device_id: "invalid".to_owned(),
                value: -1,
            },
            Reading {
                device_id: "after-error".to_owned(),
                value: 3,
            },
        ];

        assert!(insert_readings_transactionally(&pool, &readings)
            .await
            .is_err());

        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM readings")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(count.0, 0);
    }
}
