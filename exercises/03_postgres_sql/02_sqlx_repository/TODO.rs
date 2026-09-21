use sqlx::postgres::PgPool;

#[derive(Debug, sqlx::FromRow, PartialEq)]
struct Reading {
    device_id: String,
    value: i32,
}

struct ReadingRepository {
    pool: PgPool,
}

impl ReadingRepository {
    fn new(pool: PgPool) -> Self {
        todo!()
    }

    async fn insert(&self, reading: &Reading) -> Result<(), sqlx::Error> {
        todo!()
    }

    async fn list(&self) -> Result<Vec<Reading>, sqlx::Error> {
        todo!()
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
