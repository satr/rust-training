.PHONY: test reset \
	01_base_01_measurement 01_base_02_readings 01_base_03_reading_model \
	01_base_04_quality 01_base_05_parsing_errors 01_base_06_collections_iterators \
	01_base_07_hash_maps_sets 01_base_08_traits 01_base_09_error_display \
	01_base_10_reading_service 01_base_11_async_reading_service \
	01_base_12_iterator_ranges \
	02_concurrency_01_box_recursive_tree 02_concurrency_02_rc_shared_tree \
	02_concurrency_03_rc_refcell_store 02_concurrency_04_arc_mutex_threads \
	02_concurrency_05_threads_channels 02_concurrency_06_rwlock_shared_state \
	02_concurrency_07_tokio_tasks 02_concurrency_08_tokio_channels \
	02_concurrency_09_tokio_cancellation \
	02_concurrency_10_tokio_join_bounded_concurrency \
	02_concurrency_11_tokio_worker_pool \
	02_concurrency_12_tokio_watch_shutdown \
	02_concurrency_13_tokio_spawn_blocking \
	02_concurrency_14_tokio_timeouts_retries \
	03_postgres_sql_01_postgres_queries 03_postgres_sql_02_sqlx_repository \
	03_postgres_sql_03_postgres_migrations \
	03_postgres_sql_04_postgres_repository_integration \
	03_postgres_sql_05_postgres_transactions 03_postgres_sql_06_transaction_rollback \
	04_mqtt_01_mqtt_messages 04_mqtt_02_mqtt_postgres_service \
	04_mqtt_03_mqtt_batch_service 04_mqtt_04_mqtt_batch_deduplication \
	03_postgres_sql_03_postgres_migrations_infra \
	03_postgres_sql_04_postgres_repository_integration_infra \
	03_postgres_sql_05_postgres_transactions_infra \
	03_postgres_sql_06_transaction_rollback_infra \
	04_mqtt_02_mqtt_postgres_service_infra 04_mqtt_03_mqtt_batch_service_infra

test: 01_base_01_measurement 01_base_02_readings 01_base_03_reading_model 01_base_04_quality 01_base_05_parsing_errors 01_base_06_collections_iterators 01_base_07_hash_maps_sets 01_base_08_traits 01_base_09_error_display 01_base_10_reading_service 01_base_11_async_reading_service 01_base_12_iterator_ranges 02_concurrency_01_box_recursive_tree 02_concurrency_02_rc_shared_tree 02_concurrency_03_rc_refcell_store 02_concurrency_04_arc_mutex_threads 02_concurrency_05_threads_channels 02_concurrency_06_rwlock_shared_state 02_concurrency_07_tokio_tasks 02_concurrency_08_tokio_channels 02_concurrency_09_tokio_cancellation 02_concurrency_10_tokio_join_bounded_concurrency 02_concurrency_11_tokio_worker_pool 02_concurrency_12_tokio_watch_shutdown 02_concurrency_13_tokio_spawn_blocking 02_concurrency_14_tokio_timeouts_retries 03_postgres_sql_01_postgres_queries 03_postgres_sql_02_sqlx_repository 03_postgres_sql_03_postgres_migrations 03_postgres_sql_04_postgres_repository_integration 03_postgres_sql_05_postgres_transactions 03_postgres_sql_06_transaction_rollback 04_mqtt_01_mqtt_messages 04_mqtt_02_mqtt_postgres_service 04_mqtt_03_mqtt_batch_service 04_mqtt_04_mqtt_batch_deduplication

01_base_01_measurement:
	cargo test --bin 01_base_01_measurement

01_base_02_readings:
	cargo test --bin 01_base_02_readings

01_base_03_reading_model:
	cargo test --bin 01_base_03_reading_model

01_base_04_quality:
	cargo test --bin 01_base_04_quality

01_base_05_parsing_errors:
	cargo test --bin 01_base_05_parsing_errors

01_base_06_collections_iterators:
	cargo test --bin 01_base_06_collections_iterators

01_base_07_hash_maps_sets:
	cargo test --bin 01_base_07_hash_maps_sets

01_base_08_traits:
	cargo test --bin 01_base_08_traits

01_base_09_error_display:
	cargo test --bin 01_base_09_error_display

01_base_10_reading_service:
	cargo test --bin 01_base_10_reading_service

01_base_11_async_reading_service:
	cargo test --bin 01_base_11_async_reading_service

01_base_12_iterator_ranges:
	cargo test --bin 01_base_12_iterator_ranges

02_concurrency_01_box_recursive_tree:
	cargo test --bin 02_concurrency_01_box_recursive_tree

02_concurrency_02_rc_shared_tree:
	cargo test --bin 02_concurrency_02_rc_shared_tree

02_concurrency_03_rc_refcell_store:
	cargo test --bin 02_concurrency_03_rc_refcell_store

02_concurrency_04_arc_mutex_threads:
	cargo test --bin 02_concurrency_04_arc_mutex_threads

02_concurrency_05_threads_channels:
	cargo test --bin 02_concurrency_05_threads_channels

02_concurrency_06_rwlock_shared_state:
	cargo test --bin 02_concurrency_06_rwlock_shared_state

02_concurrency_07_tokio_tasks:
	cargo test --bin 02_concurrency_07_tokio_tasks

02_concurrency_08_tokio_channels:
	cargo test --bin 02_concurrency_08_tokio_channels

02_concurrency_09_tokio_cancellation:
	cargo test --bin 02_concurrency_09_tokio_cancellation

02_concurrency_10_tokio_join_bounded_concurrency:
	cargo test --bin 02_concurrency_10_tokio_join_bounded_concurrency

02_concurrency_11_tokio_worker_pool:
	cargo test --bin 02_concurrency_11_tokio_worker_pool

02_concurrency_12_tokio_watch_shutdown:
	cargo test --bin 02_concurrency_12_tokio_watch_shutdown

02_concurrency_13_tokio_spawn_blocking:
	cargo test --bin 02_concurrency_13_tokio_spawn_blocking

02_concurrency_14_tokio_timeouts_retries:
	cargo test --bin 02_concurrency_14_tokio_timeouts_retries

03_postgres_sql_01_postgres_queries:
	cargo test --bin 03_postgres_sql_01_postgres_queries

03_postgres_sql_02_sqlx_repository:
	cargo test --bin 03_postgres_sql_02_sqlx_repository

03_postgres_sql_03_postgres_migrations:
	cargo test --bin 03_postgres_sql_03_postgres_migrations

03_postgres_sql_04_postgres_repository_integration:
	cargo test --bin 03_postgres_sql_04_postgres_repository_integration

03_postgres_sql_05_postgres_transactions:
	cargo test --bin 03_postgres_sql_05_postgres_transactions

03_postgres_sql_06_transaction_rollback:
	cargo test --bin 03_postgres_sql_06_transaction_rollback

04_mqtt_01_mqtt_messages:
	cargo test --bin 04_mqtt_01_mqtt_messages

04_mqtt_02_mqtt_postgres_service:
	cargo test --bin 04_mqtt_02_mqtt_postgres_service

04_mqtt_03_mqtt_batch_service:
	cargo test --bin 04_mqtt_03_mqtt_batch_service

04_mqtt_04_mqtt_batch_deduplication:
	cargo test --bin 04_mqtt_04_mqtt_batch_deduplication

# Starts an isolated PostgreSQL instance, runs ignored database tests, and
# removes containers, volumes, and orphan services even when a test fails.
03_postgres_sql_03_postgres_migrations_infra:
	./scripts/test_infra.sh 15_postgres_migrations

03_postgres_sql_04_postgres_repository_integration_infra:
	./scripts/test_infra.sh 16_postgres_repository_integration

03_postgres_sql_05_postgres_transactions_infra:
	./scripts/test_infra.sh 17_postgres_transactions

03_postgres_sql_06_transaction_rollback_infra:
	./scripts/test_infra.sh 18_transaction_rollback

04_mqtt_02_mqtt_postgres_service_infra:
	./scripts/test_infra.sh 19_mqtt_postgres_service

04_mqtt_03_mqtt_batch_service_infra:
	./scripts/test_infra.sh 20_mqtt_batch_service

reset:
	@printf "This will overwrite all active exercises/*/*/main.rs task files from their TODO.rs copies. Continue? [y/N] "; \
	read answer; \
	case "$$answer" in \
		y|Y|yes|YES) \
			for todo in exercises/*/*/TODO.rs; do \
				target="$${todo%/TODO.rs}/main.rs"; \
				cp "$$todo" "$$target"; \
				echo "Reset $$target"; \
			done ;; \
		*) echo "Reset cancelled."; exit 1 ;; \
	esac
