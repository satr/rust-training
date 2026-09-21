use sqlx::postgres::PgPool;

#[derive(Debug, sqlx::FromRow, PartialEq)]
struct Reading {
    device_id: String,
    value: i32,
}

struct ReadingRepository {
    pool: PgPool,
}

#[allow(dead_code)]
impl ReadingRepository {
    fn new(pool: PgPool) -> Self {
        ReadingRepository { pool }
    }

    async fn insert(&self, reading: &Reading) -> Result<(), sqlx::Error> {
        sqlx::query(insert_sql())
            .bind(&reading.device_id)
            .bind(&reading.value)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn list(&self) -> Result<Vec<Reading>, sqlx::Error> {
        let list = sqlx::query_as::<_, Reading>(select_sql())
            .fetch_all(&self.pool)
            .await?;
        Ok(list)
    }
}

fn insert_sql() -> &'static str {
    "INSERT INTO readings (device_id, value) VALUES ($1, $2)"
}

fn select_sql() -> &'static str {
    "SELECT device_id, value FROM readings ORDER BY device_id"
}

fn main() {
    println!("insert: {}", insert_sql());
    println!("select: {}", select_sql());
}

#[cfg(test)]
mod tests {
    use super::{insert_sql, select_sql};

    #[test]
    fn uses_parameterized_insert_query() {
        assert_eq!(
            insert_sql(),
            "INSERT INTO readings (device_id, value) VALUES ($1, $2)"
        );
    }

    #[test]
    fn selects_rows_in_a_stable_order() {
        assert_eq!(
            select_sql(),
            "SELECT device_id, value FROM readings ORDER BY device_id"
        );
    }
}
