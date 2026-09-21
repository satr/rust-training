# Exercise index

This table of contents groups the exercises by the top-level folders under
[`exercises/`](.). Each item links to the exercise README with the theory,
task, and run instructions.

## 01_base

- [Exercise 01: Measurements](01_base/01_measurement/README.md)
- [Exercise 02: Mutable readings](01_base/02_readings/README.md)
- [Exercise 03: Reading model](01_base/03_reading_model/README.md)
- [Exercise 04: Reading quality](01_base/04_quality/README.md)
- [Exercise 05: Custom parsing errors](01_base/05_parsing_errors/README.md)
- [Exercise 06: Collections and iterators](01_base/06_collections_iterators/README.md)
- [Exercise 07: Hash maps and sets](01_base/07_hash_maps_sets/README.md)
- [Exercise 08: Traits and generic functions](01_base/08_traits/README.md)
- [Exercise 09: Standard error formatting](01_base/09_error_display/README.md)
- [Exercise 10: In-memory reading service](01_base/10_reading_service/README.md)
- [Exercise 11: Async reading service](01_base/11_async_reading_service/README.md)
- [Exercise 12: Iterator ranges and slices](01_base/12_iterator_ranges/README.md)

## 02_concurrency

- [Exercise 22: `Box<T>` and recursive trees](02_concurrency/01_box_recursive_tree/README.md)
- [Exercise 23: `Rc<T>` and shared ownership](02_concurrency/02_rc_shared_tree/README.md)
- [Exercise 24: `Rc<RefCell<T>>` and interior mutability](02_concurrency/03_rc_refcell_store/README.md)
- [Exercise 25: `Arc<Mutex<T>>` and threads](02_concurrency/04_arc_mutex_threads/README.md)
- [Exercise 26: Threads and channels](02_concurrency/05_threads_channels/README.md)
- [Exercise 27: `RwLock` for shared read-heavy state](02_concurrency/06_rwlock_shared_state/README.md)
- [Exercise 28: Tokio tasks and `JoinHandle`](02_concurrency/07_tokio_tasks/README.md)
- [Exercise 29: Tokio `mpsc` channels](02_concurrency/08_tokio_channels/README.md)
- [Exercise 30: Tokio cancellation with `select!`](02_concurrency/09_tokio_cancellation/README.md)
- [Exercise 31: Tokio `join!` with bounded concurrency](02_concurrency/10_tokio_join_bounded_concurrency/README.md)
- [Exercise 32: Tokio worker pool with `mpsc` and `select!`](02_concurrency/11_tokio_worker_pool/README.md)
- [Exercise 33: Tokio `watch` for graceful worker shutdown](02_concurrency/12_tokio_watch_shutdown/README.md)
- [Exercise 34: Tokio `spawn_blocking` for blocking work](02_concurrency/13_tokio_spawn_blocking/README.md)
- [Exercise 35: Tokio timeouts and retries](02_concurrency/14_tokio_timeouts_retries/README.md)

## 03_postgres_sql

- [Exercise 13: PostgreSQL query boundaries](03_postgres_sql/01_postgres_queries/README.md)
- [Exercise 14: SQLx PostgreSQL repository](03_postgres_sql/02_sqlx_repository/README.md)
- [Exercise 15: PostgreSQL migrations](03_postgres_sql/03_postgres_migrations/README.md)
- [Exercise 16: PostgreSQL repository integration](03_postgres_sql/04_postgres_repository_integration/README.md)
- [Exercise 17: PostgreSQL transactions](03_postgres_sql/05_postgres_transactions/README.md)
- [Exercise 18: Transaction rollback](03_postgres_sql/06_transaction_rollback/README.md)

## 04_mqtt

- [Exercise 12: MQTT-style message parsing](04_mqtt/01_mqtt_messages/README.md)
- [Exercise 19: MQTT to PostgreSQL service flow](04_mqtt/02_mqtt_postgres_service/README.md)
- [Exercise 20: MQTT batch service](04_mqtt/03_mqtt_batch_service/README.md)
- [Exercise 21: Deduplicating an MQTT batch](04_mqtt/04_mqtt_batch_deduplication/README.md)
