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
    value INTEGER NOT NULL
)
"#;

const INSERT_READING_COMMAND: &str = r#"
INSERT INTO readings (device_id, value) VALUES ($1, $2)
"#;

fn parse_mqtt_reading(topic: &str, payload: &str) -> Result<Reading, ServiceError> {
    // Expected topic shape: devices/{device_id}/readings.
    let mut parts = topic.split('/');
    let prefix = parts.next();
    let device_id = parts.next();
    let suffix = parts.next();
    let extra = parts.next();
    if prefix != Some("devices")
        || suffix != Some("readings")
        || device_id.is_none_or(str::is_empty)
        || extra.is_some()
    {
        return Err(ServiceError::InvalidTopic);
    }
    let device_id = device_id.ok_or(ServiceError::InvalidTopic)?;
    let value = payload
        .trim()
        .parse::<i32>()
        .map_err(|_| ServiceError::InvalidPayload)?;
    Ok(Reading {
        device_id: device_id.to_owned(),
        value,
    })
}

async fn process_mqtt_message(
    pool: &PgPool,
    topic: &str,
    payload: &str,
) -> Result<(), ServiceError> {
    let reading = parse_mqtt_reading(topic, payload)?;
    let mut transaction = pool.begin().await.map_err(|_| ServiceError::Database)?;
    sqlx::query(INSERT_READING_COMMAND)
        .bind(&reading.device_id)
        .bind(reading.value)
        .execute(&mut *transaction)
        .await
        .map_err(|_| ServiceError::Database)?;
    transaction
        .commit()
        .await
        .map_err(|_| ServiceError::Database)?;
    Ok(())
}

#[cfg(test)]
#[path = "../../test_support/postgres.rs"]
mod postgres;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("MQTT to PostgreSQL service exercise");
}

#[cfg(test)]
mod tests {
    use super::{parse_mqtt_reading, postgres, process_mqtt_message, Reading, ServiceError};

    #[test]
    fn parses_valid_topic_and_payload() {
        assert_eq!(
            parse_mqtt_reading("devices/sensor-1/readings", "42"),
            Ok(Reading {
                device_id: "sensor-1".to_owned(),
                value: 42,
            })
        );
    }

    #[test]
    fn rejects_invalid_topic_shape() {
        assert_eq!(
            parse_mqtt_reading("devices/sensor-1/status", "42"),
            Err(ServiceError::InvalidTopic)
        );
    }

    #[test]
    fn rejects_invalid_payload() {
        assert_eq!(
            parse_mqtt_reading("devices/sensor-1/readings", "not-a-number"),
            Err(ServiceError::InvalidPayload)
        );
    }

    #[test]
    fn rejects_empty_device_id() {
        assert_eq!(
            parse_mqtt_reading("devices//readings", "42"),
            Err(ServiceError::InvalidTopic)
        );
    }

    #[tokio::test(flavor = "current_thread")]
    #[ignore = "requires DATABASE_URL and a running PostgreSQL instance"]
    async fn stores_a_valid_mqtt_message() {
        let pool = postgres::postgres_pool().await;
        sqlx::query(super::CREATE_READINGS_TABLE)
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("TRUNCATE readings")
            .execute(&pool)
            .await
            .unwrap();

        process_mqtt_message(&pool, "devices/sensor-1/readings", "42")
            .await
            .unwrap();

        let row: (String, i32) =
            sqlx::query_as("SELECT device_id, value FROM readings ORDER BY device_id LIMIT 1")
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(row, ("sensor-1".to_owned(), 42));
    }
}
