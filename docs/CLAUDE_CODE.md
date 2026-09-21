# Using Claude Code for Rust learning

Claude Code is an agentic coding assistant that can read this repository,
explain Rust concepts, edit exercise files, run Cargo and Makefile commands,
and review compiler or test failures.

See the [official Claude Code documentation](https://code.claude.com/docs/en)
for installation and current usage details.

## Start Claude Code

From the repository root:

```bash
cd /path/to/rust-training
claude
```

Claude Code uses `CLAUDE.md` files for persistent project instructions. This
repository's primary instructions are in [`AGENTS.md`](../AGENTS.md), so begin a
session with:

```text
Read AGENTS.md, README.md, and the README for the current exercise before
changing anything. Follow the exercise structure and preserve TODO.rs.
```

## Learn and review

Ask for explanations before asking for implementation:

```text
Explain why `yield_now().await` is useful in this exercise.
Compare it with Go's scheduler and C# task yielding. Do not edit files.
```

After implementing an exercise yourself:

```text
Review only exercises/02_concurrency/09_tokio_cancellation/main.rs.
Do not modify TODO.rs or reveal a complete replacement. Explain the first
correctness issue, then run the focused Makefile test.
```

When a fix is wanted, request it explicitly:

```text
Apply the smallest fix for the issue you identified. Preserve TODO.rs and run
the focused test plus rustfmt.
```

## Generate an exercise

```text
Read AGENTS.md and the progress log. Create the next Rust learning exercise
for Tokio select! and cancellation in the correct topic group. Create
main.rs, an identical untouched TODO.rs, and README.md with theory, a worked
example, the task, expected behavior, and tests. Register the binary and
Makefile target. Do not put the final solution in README.md. Verify the
scaffold without changing existing exercises.
```

Use `make test` for the repository suite and a group-prefixed target for one
exercise, for example `make 02_concurrency_09_tokio_cancellation`.

Claude Code can solve tasks very aggressively. For training, explicitly ask
for hints, explanations, or review before asking it to edit `main.rs`.
