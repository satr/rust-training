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

const INSERT_READING_COMMAND: &str = r#"
INSERT INTO readings (device_id, value) VALUES ($1, $2)
"#;

const SELECT_READINGS_COMMAND: &str = r#"
SELECT * FROM readings ORDER BY device_id
"#;

async fn ensure_schema(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(CREATE_READINGS_TABLE).execute(pool).await?;
    Ok(())
}

async fn insert_reading(pool: &PgPool, reading: &Reading) -> Result<(), sqlx::Error> {
    sqlx::query(INSERT_READING_COMMAND)
        .bind(&reading.device_id)
        .bind(&reading.value)
        .execute(pool)
        .await?;
    Ok(())
}

async fn list_readings(pool: &PgPool) -> Result<Vec<Reading>, sqlx::Error> {
    let list = sqlx::query_as::<_, Reading>(SELECT_READINGS_COMMAND)
        .fetch_all(pool)
        .await?;
    Ok(list)
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
    use super::{
        ensure_schema, insert_reading, list_readings, postgres, Reading, CREATE_READINGS_TABLE,
    };

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
