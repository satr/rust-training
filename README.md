# Rust software language training

This repository is a practical, exercise-driven Rust training space for an experienced software engineer coming from other OOP languages.

The goal is to become comfortable writing typical Rust code for backend services in a few days. Topics will gradually include:

[Browsable exercise index](exercises/README.md) grouped by topic

## Learning workflow

Each topic follows this pattern:

1. Short theory explanation
2. Small worked example
3. Focused coding task
4. Compile, run, and review
5. Progress recorded for the next session

For the full exercise table of contents, open [`exercises/README.md`](exercises/README.md).

Each exercise has its own directory under a topic group in [`exercises/`](exercises/) containing `main.rs`, `TODO.rs`, and `README.md`. Current groups are `01_base`, `02_concurrency`, `03_postgres_sql`, and `04_mqtt`; exercise numbering restarts at `01_` inside each group. `main.rs` is the active solution, `TODO.rs` is the untouched reset copy, and `README.md` contains the theory, task, expected behavior, and follow-up explanations.

Each completed task also has a `TODO.rs` copy with stubs and tests for repeating the exercise later. These reset files are kept outside the normal Makefile workflow; run and test the active `main.rs` files through their regular targets.

Reset all active task implementations to their `_todo.rs` copies with command `make reset`. This overwrites the current `.rs` solutions and asks for confirmation:

New exercises should be added as `exercises/<group>/<NN_task>/` containing `main.rs`, `TODO.rs`, and `README.md`, where `NN` restarts at `01` within that group. Register each binary in `Cargo.toml` and add a matching Makefile test target named `<group>_<NN_task>` without a `test_` prefix. Keep the top level of `exercises/` limited to topic groups and shared support directories.

The persistent training context and progress log are in [`AGENTS.md`](AGENTS.md).

## Installation

See [`INSTALL.md`](INSTALL.md) for the tools and environment required to work
with this repository, including Rust, Make, and Docker.

## Using AI coding assistants

This repository can be used with major coding assistants. Keep
[`AGENTS.md`](AGENTS.md) as the authoritative training and repository workflow
instructions, and ask the assistant to preserve `TODO.rs`, explain concepts
before solving tasks, and run the focused Makefile target.

- [Codex](CODEX.md) — repository workflow, exercise generation, explanations,
  and code review guidance.
- [Claude Code](docs/CLAUDE_CODE.md) — terminal, IDE, desktop, and web agent
- [GitHub Copilot](docs/GITHUB_COPILOT.md) — inline completion, chat, and GitHub agent
- [Gemini CLI](docs/GEMINI_CLI.md) — terminal-based coding assistant
- [Cursor](docs/CURSOR.md) — editor-based chat and agent workflow

## Running tests

Check all registered binaries:

```bash
cargo check
```

Run all exercise tests:

```bash
make test
```

Run one exercise's tests:

```bash
make 01_base_01_measurement
make 01_base_02_readings
#etc.
```

Run tests that require PostgreSQL. This starts the Docker Compose database,
sets `DATABASE_URL`, runs the ignored integration tests, and shuts the
container down and deletes its resources afterward (including when the tests
fail):

```bash
make 03_postgres_sql_03_postgres_migrations_infra
make 03_postgres_sql_04_postgres_repository_integration_infra
#etc.
```

Infrastructure targets run both regular tests and tests marked `#[ignore]` by
passing Cargo's `--include-ignored` option.

Docker Compose is configured in [`docker-compose.yml`](docker-compose.yml).
It exposes PostgreSQL on port `5433` so it is less likely to conflict with a
locally installed PostgreSQL server. The reusable helper is
[`scripts/test_infra.sh`](scripts/test_infra.sh); it receives a Cargo binary
name and deletes containers, volumes, and orphan services after testing.
