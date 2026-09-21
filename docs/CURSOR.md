# Using Cursor for Rust learning

Cursor is an editor-based AI coding assistant with inline chat, codebase
search, and agent workflows. It can be used for the same explain, implement,
review, and test cycle as the other assistants in this repository.

See the [official Cursor rules documentation](https://cursor.com/docs) for
current editor and project-rule configuration.

## Project instructions

Cursor project rules are stored in version-controlled files under
`.cursor/rules`. The repository's primary instructions remain in
[`AGENTS.md`](../AGENTS.md). Add a Cursor rule only if you want Cursor-specific
behavior, such as always asking for a hint before editing an exercise.

Suggested rule text:

```text
Read AGENTS.md and the relevant exercise README before making changes.
This is a Rust training repository. Preserve TODO.rs, review only active
main.rs unless asked to implement, and run the focused Makefile target.
Prefer explanations and hints before complete solutions.
```

## Explain and review code

Select a function or error in the editor and ask:

```text
Explain this code line by line. Focus on ownership, borrowing, and why the
compiler accepts or rejects it. Compare the design with C# and Go. Do not edit.
```

For a complete exercise review:

```text
Review only exercises/03_postgres_sql/06_transaction_rollback/main.rs.
Check transaction lifetime, rollback behavior, error propagation, and tests.
Do not modify TODO.rs. Give findings and hints before proposing changes.
```

## Generate an exercise

```text
Read AGENTS.md, README.md, and the progress log. Create the next exercise in
the correct topic group with main.rs, identical TODO.rs, README.md, tests,
Cargo.toml registration, and a group-prefixed Makefile target. Explain the
theory and worked example, but do not include the final solution. Preserve all
existing exercises and run formatting and focused tests.
```

## Verify changes

After accepting an edit, inspect the diff and run:

```bash
cargo check
make 03_postgres_sql_06_transaction_rollback
make test
```

Cursor may suggest or apply broad changes when its Agent mode is used. Keep
the task scope explicit and confirm that only the active `main.rs` changed
when working on an exercise.
