use sqlx::postgres::PgPool;

#[derive(Debug, PartialEq)]
struct Reading {
    device_id: String,
    value: i32,
}

#[derive(Debug, PartialEq)]
enum ServiceError {
    InvalidTopic,
    InvalidPayload,
    Database,
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

fn parse_mqtt_reading(topic: &str, payload: &str) -> Result<Reading, ServiceError> {
    // Expected topic shape: devices/{device_id}/readings.
    todo!()
}

async fn process_mqtt_batch(pool: &PgPool, messages: &[(&str, &str)]) -> Result<(), ServiceError> {
    // Parse the complete batch before opening the database transaction.
    todo!()
}

#[cfg(test)]
#[path = "../../test_support/postgres.rs"]
mod postgres;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("MQTT batch service exercise");
}

#[cfg(test)]
mod tests {
    use super::{parse_mqtt_reading, postgres, process_mqtt_batch, Reading, ServiceError};

    #[test]
    fn parses_a_valid_reading() {
        assert_eq!(
            parse_mqtt_reading("devices/sensor-1/readings", "42"),
            Ok(Reading {
                device_id: "sensor-1".to_owned(),
                value: 42,
            })
        );
    }

    #[test]
    fn rejects_invalid_topic() {
        assert_eq!(
            parse_mqtt_reading("devices//readings", "42"),
            Err(ServiceError::InvalidTopic)
        );
    }

    #[test]
    fn rejects_invalid_payload() {
        assert_eq!(
            parse_mqtt_reading("devices/sensor-1/readings", "bad"),
            Err(ServiceError::InvalidPayload)
        );
    }

    #[tokio::test(flavor = "current_thread")]
    #[ignore = "requires DATABASE_URL and a running PostgreSQL instance"]
    async fn stores_the_whole_valid_batch() {
        let pool = postgres::postgres_pool().await;
        sqlx::query(super::CREATE_READINGS_TABLE)
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("TRUNCATE readings")
            .execute(&pool)
            .await
            .unwrap();

        process_mqtt_batch(
            &pool,
            &[
                ("devices/sensor-1/readings", "10"),
                ("devices/sensor-2/readings", "20"),
            ],
        )
        .await
        .unwrap();

        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM readings")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(count.0, 2);
    }
}
