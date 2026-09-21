# Using Gemini CLI for Rust learning

Gemini CLI is a terminal-based coding assistant that can inspect the
repository, explain Rust, edit files, run commands, and help diagnose Cargo
failures.

See the [official Gemini CLI documentation](https://github.com/google-gemini/gemini-cli)
for installation and current usage details.

## Start Gemini CLI

From the repository root, start the CLI using its installed command:

```bash
cd /path/to/rust-training
gemini
```

Gemini CLI normally loads project instructions from `GEMINI.md`. It can also
be configured to load `AGENTS.md`; if that is not enabled in your installation,
begin each session with:

```text
Read AGENTS.md, README.md, and the current exercise README. Follow all
repository rules, especially preserving TODO.rs and using the Makefile tests.
```

## Ask for explanations and reviews

```text
Explain the difference between JoinHandle, a future, and a Tokio task in the
current exercise. Use a small example and compare it with Go goroutines. Do
not edit files.
```

```text
Review exercises/02_concurrency/07_tokio_tasks/main.rs.
Run only its focused test, explain the ownership and async issues, and give
hints before suggesting code. Do not modify TODO.rs.
```

## Generate an exercise

```text
Read AGENTS.md and the progress log. Add the next focused Rust training
exercise in the appropriate group. Create main.rs, identical TODO.rs, and
README.md with theory, a worked example, the task, expected behavior, and
tests. Register it in Cargo.toml and Makefile using the repository naming
rules. Keep the final solution out of the README and verify the scaffold.
```

## Run the checks

Use the repository's commands rather than ad-hoc `rustc` commands:

```bash
cargo check
make 02_concurrency_07_tokio_tasks
make test
```

Gemini CLI can use hierarchical `GEMINI.md` files if you want a compatibility
wrapper around the repository rules. Keep any wrapper short and point it to
`AGENTS.md` as the authoritative instructions.

