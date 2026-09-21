#[derive(Debug, PartialEq)]
struct Reading {
    device_id: String,
    value: i32,
}

fn insert_reading_sql() -> &'static str {
    todo!()
}

fn select_readings_sql() -> &'static str {
    todo!()
}

fn main() {
    println!("insert: {}", insert_reading_sql());
    println!("select: {}", select_readings_sql());
}

#[cfg(test)]
mod tests {
    use super::{insert_reading_sql, select_readings_sql};

    #[test]
    fn builds_parameterized_insert_query() {
        assert_eq!(
            insert_reading_sql(),
            "INSERT INTO readings (device_id, value) VALUES ($1, $2)"
        );
    }

    #[test]
    fn builds_select_query() {
        assert_eq!(
            select_readings_sql(),
            "SELECT device_id, value FROM readings ORDER BY device_id"
        );
    }
}
