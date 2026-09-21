# Rust learning and training context

The user is an experienced software engineer with many years of experience in C# and Go, and is somewhat familiar with basic Rust.

This repository is used for a practical, exercise-driven Rust learning and training program. The work includes writing backend services in Rust, potentially integrating with MQTT and PostgreSQL, as well as solving typical coding tasks.

## Teaching approach

- Teach gradually through small, runnable coding exercises rather than long documentation or theory-first explanations.
- For each new concept, first give a concise theory explanation and a small worked example, then give a focused task. Do not ask the user to guess unfamiliar Rust concepts from compiler or autocomplete suggestions.
- Ask the user to repeat or extend code so concepts become memorable.
- Relate Rust concepts to the user's C# and Go experience when useful, while clearly calling out important differences.
- Prioritize idiomatic, production-oriented Rust for backend services: ownership and borrowing, error handling, traits, async code, testing, MQTT, PostgreSQL, configuration, observability, and clean project structure.
- Include practical coding exercises and explain the reasoning, complexity, and Rust-specific implementation choices.
- Keep sessions focused and practical; avoid sending the user away to read documentation unless it is genuinely necessary. There is no fixed time limit: expand the user's Rust knowledge rather than stopping after the initial backend learning path.
- When changing the repository, prefer complete examples and verify them with formatting, tests, or compilation as appropriate.

At the beginning of a new training session, briefly recall this context and continue from the current state of the exercises in the repository.

## Persistent training workflow

- Keep each task in its own `exercises/<group>/<NN_task>/` directory with exactly `main.rs` for the active implementation, `TODO.rs` for the untouched reset copy, and `README.md` for the theory, task, and follow-up notes. Restart task numbering at `01_` inside every group. Keep the top level of `exercises/` limited to numbered topic groups and shared support directories.
- At the start of every new iteration, create both `exercises/<group>/<NN_task>/main.rs` and `exercises/<group>/<NN_task>/TODO.rs` as identical files, plus `exercises/<group>/<NN_task>/README.md`. The Rust files must contain the exercise structure and `todo!()` stub methods. The Markdown file must immediately contain the theory, worked-example context when useful, task description, expected behavior, and run instructions, so the user can pause, commit, and resume later without relying on chat history.
- For every newly introduced Rust component or syntax, put the necessary working theory, lifecycle/ownership rules, common usage patterns, and relevant pitfalls in that task's `README.md`. Do not provide the final task solution in the README; keep the implementation for the user to write.
- The user implements and receives review only for `main.rs`; preserve `TODO.rs` as the untouched reset copy for repeating the task later.
- Include unit tests in every new task `.rs` file that encode the expected behavior. Keep the tests alongside the stubs so the user can run them while implementing the task.
- Register every new task binary in `Cargo.toml` using the same `group_task` name as the Makefile target (for example, `01_base_01_measurement`), with the path pointing at `exercises/<group>/<NN_task>/main.rs`. Keep an aggregate `make test` target covering all active exercises; do not add `cargo run` targets.
- Keep `README.md` command examples concise. Do not expand the long lists of every task command; use an `# etc.` marker or refer to the Makefile. Put detailed commands for a specific exercise in that exercise's `exercises/<group>/<task>/README.md`.
- Put reusable database-test setup in `exercises/test_support/` and include it from task test modules instead of duplicating `DATABASE_URL` and `PgPoolOptions` setup.
- Add Makefile test targets only for active `<task>/main.rs`, never for `TODO.rs`; TODO files are reset copies and are not part of the normal Makefile workflow. Name each target `<group>_<task>` without a `test_` prefix.
- Provide a `make reset` target that asks for explicit confirmation before copying every `exercises/*/*/TODO.rs` file over its corresponding `exercises/*/*/main.rs` task file. This reset must not modify Markdown, Cargo, or Makefile files.
- At the end of each iteration, update the Markdown companion with the user's implementation outcome, compiler errors, corrections, explanations, and key takeaways.
- Use the repository's `Cargo.toml`, `Cargo.lock`, and `Makefile` to build and run exercises when available; prefer `cargo check`, `cargo test`, and the relevant Make target over standalone `rustc` commands.
- For tests requiring external infrastructure, use a reusable script under `scripts/` and a group-prefixed Makefile target named `<group>_<task>_infra`. The script must clean up with `docker compose down --volumes --remove-orphans`, including after failures.
- After meaningful exercises or sessions, update the progress log below with what was practiced, what the user can do independently, and the next recommended step.
- Use the progress log to resume training after a Codex restart; do not make the user repeat their background or completed topics.

## Extended curriculum priorities

After the current MQTT/PostgreSQL path, deliberately extend the curriculum with
the following Rust fundamentals and backend patterns. Teach each topic with a
short theory explanation, a small example, a task, and tests:

- Smart pointers and ownership: `Box<T>`, `Rc<T>`, `Arc<T>`, `Cell<T>`, and `RefCell<T>`; explain heap allocation, shared ownership, interior mutability, and when each is appropriate.
- Concurrency foundations: `std::thread`, `JoinHandle`, `move` closures, `Send`, `Sync`, channels, `Arc<Mutex<T>>`, and `RwLock`.
- Async concurrency: Tokio tasks, `spawn`, channels, cancellation, `join!`, `select!`, bounded concurrency, and avoiding blocking work inside async tasks.
- Rust memory and lifetime basics: stack versus heap, borrowing across threads/tasks, owned versus borrowed data, lifetimes in practical function signatures, and trait objects (`dyn Trait`).
- Backend essentials: configuration, graceful shutdown, timeouts, retries, backpressure, structured errors, logging/tracing, and clean separation of domain/application/infrastructure code.
- Database and messaging patterns: idempotency, connection-pool limits, transaction boundaries, MQTT consumer loops, acknowledgements, ordering, and failure handling.
- Coding practice: common collections/iterator problems, parsing, ownership transformations, graph/tree basics, complexity analysis, and writing focused tests.

Prefer the next exercises to cover these topics instead of only polishing
already familiar SQLx patterns. Relate the concepts to C# (`class` references,
`Task`, locks, channels) and Go (pointers, goroutines, channels, mutexes), while
highlighting where Rust's compile-time ownership and `Send`/`Sync` checks differ.

## Progress log

- Status: Training has just started.
- Background: Experienced in C# and Go; familiar with basic Rust.
- Completed: First measurement exercise; practiced `&str`, slices, `Result`, `Option`, parsing, mutation, numeric casts, and compiler-guided fixes. Second exercise; practiced `&mut Vec<i32>`, mutable versus immutable borrows, `Result<(), String>`, `map_err`, and the `?` operator. Third exercise; practiced structs, `Option` to `Result` conversion with `ok_or_else`, tuple destructuring, owned `String` fields, and validation. Fourth exercise; practiced enums, exhaustive `match`, borrowed enum arguments, and unit tests. Fifth exercise; practiced custom error enums, exact error matching in tests, and mapping parser failures into domain errors.
- Completed: Sixth exercise; practiced `Vec`, borrowed slices, lazy iterator chains, `filter`, `map`, `collect`, `sum`, cloning owned `String` values from borrowed structs, and numeric casts for averages. All four exercise tests pass.
- Completed: Seventh exercise; practiced `HashSet`, `HashMap`, duplicate detection, frequency counting, `entry`, mutable-reference dereferencing, and owned map keys. Eighth exercise; practiced traits, trait implementations, `&self`, generic bounds, iterator mapping, and collection of owned results. All current exercise tests pass.
- Completed: Ninth exercise; practiced `Display`, `std::error::Error`, exact user-facing error messages, and testing `.to_string()` output. Tenth exercise; practiced service-owned state, `impl` methods, `&mut self` versus `&self`, parsing into domain structs, and iterator-based aggregation.
- Completed: Eleventh exercise; practiced async methods, `.await`, Tokio runtime/test setup, and async unit tests.
- Completed: Twelfth exercise; practiced MQTT-style topic parsing, iterator-based segment validation, variable shadowing from `Option<&str>` to `&str`, payload parsing, and domain errors.
- Completed: Thirteenth exercise; practiced PostgreSQL parameterized SQL, `$1`/`$2` placeholders, fixed string literals with `&'static str`, and the difference between borrowed literals and owned `String` values.
- Completed: Fourteenth exercise; practiced `sqlx`, `PgPool` borrowing, `FromRow`, async repository methods, parameter binding, `query_as`, and propagating database errors with `?`.
- Completed: Exercise 15; practiced executing repeatable PostgreSQL schema setup through SQLx and running ignored integration tests with temporary Docker infrastructure.
- Completed: Exercise 16; practiced a real PostgreSQL repository round trip, parameterized insert, query mapping, and shared database-test setup.
- Completed: Exercise 17; practiced SQLx transactions, mutable executor reborrowing with `&mut *transaction`, parameterized batch inserts, and atomic commit behavior.
- Completed: Exercise 18; practiced transaction rollback when a later insert fails a database constraint, and serialized shared-database integration tests to avoid concurrent schema setup races. Focused tests and formatting checks pass; Docker-backed integration tests still need to run in an environment with Docker access.
- Completed: Exercise 20; practiced MQTT batch validation, `collect::<Result<_, _>>()?`, non-negative payload validation, and one atomic PostgreSQL transaction. Unit tests pass.
- Completed: Exercise 21; practiced MQTT batch deduplication with `HashMap`, owned readings, deterministic sorting, and all-or-nothing parsing. All three unit tests pass.
- Completed: Exercise 22; practiced `Box<T>` for recursive enums, borrowed recursive methods, and maximum-depth traversal. All three unit tests pass.
- Completed: Exercise 23; practiced single-threaded shared ownership with `Rc<T>`, reference counts, and shared recursive subtrees. Both unit tests pass.
- Completed: Exercise 24; practiced `Rc<RefCell<T>>` for single-threaded shared mutable state and runtime borrow checking. Both unit tests pass.
- Completed: Exercise 25; practiced `Arc<Mutex<T>>`, `std::thread`, `move` closures, and preserving shared updates across workers. All three unit tests pass.
- Completed: Exercise 26; practiced `std::sync::mpsc`, worker threads, sender cloning, receiver ownership, and collecting concurrent results. All three unit tests pass.
- Completed: Exercise 27; practiced `Arc<RwLock<Vec<i32>>>`, read/write guards, writer and reader thread ownership, and the difference between `RwLock` and `Mutex`. All three unit tests pass.
- Completed: Exercise 28; practiced Tokio tasks, `tokio::spawn`, `JoinHandle`, `async move`, and returning owned results from async workers. All three unit tests pass.
- Prepared: Exercise 29 scaffolded; the task introduces Tokio `mpsc` channels, sender cloning, channel closure, and bounded-channel backpressure.
- Completed: Exercise 29; practiced Tokio bounded `mpsc`, cloned senders, channel closure, backpressure, and awaiting worker handles after receiving. All three unit tests pass.
- Completed: Exercise 30; practiced cooperative cancellation with `oneshot`, `tokio::select!`, async worker futures, and `yield_now().await`. All three unit tests pass.
- Prepared: Exercise 31 scaffolded; the task introduces Tokio `join!`, bounded concurrency, and avoiding blocking work inside async tasks.
- Completed: Added the grouped exercise index in `exercises/README.md` and linked it from the root README for faster navigation across the curriculum.
- Prepared: Exercise 12 scaffolded; the task compares `iter().take(to_id).skip(from_id)` with the half-open slice range `[from_id..to_id]`.
- Next curriculum direction: continue with Tokio `join!`, bounded concurrency, and avoiding blocking work inside async tasks.
