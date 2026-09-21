# Using Codex for Rust learning

Codex is an AI coding assistant that can work with this repository. It can
read the existing exercises, explain Rust concepts, create new learning tasks,
review implementations, run commands, and help diagnose compiler or test
failures.

## Install and start Codex

Follow the [official Codex CLI installation guide](https://developers.openai.com/codex/cli/)
for the current installation options. Do not run installer commands copied from
untrusted sources; use only the commands documented on the official page.

Verify the installation:

```bash
codex --version
codex doctor
```

Start Codex from the repository root:

```bash
cd /path/to/rust-training
codex
```

Or select the repository explicitly from another directory:

```bash
codex --cd /path/to/rust-training
```

Authenticate on first use if needed:

```bash
codex login
```

Codex reads [`AGENTS.md`](AGENTS.md) and uses its instructions for the
training context, exercise structure, progress tracking, and verification
commands. To keep terminal scrollback visible, use:

```bash
codex --no-alt-screen --cd /path/to/rust-training
```

## Ask for explanations

Ask Codex to explain a specific Rust concept or a specific piece of code. Give
it the context you want and say whether you want an explanation only or code
changes as well.

Examples:

```text
Explain why this value cannot be moved into two spawned Tokio tasks.
Compare the Rust ownership rules here with C# references and Go pointers.
Do not modify files; explain the compiler error and show a small example.
```

```text
Explain the `tokio::select!` in Exercise 30 line by line.
Focus on futures, polling, cancellation, and why `yield_now().await` is used.
Do not give me the completed exercise yet.
```

For effective learning, ask Codex to explain the idea first, implement the
task yourself, and then request a review. You can also ask it to provide a
smaller worked example before returning to the exercise.

## Generate a new exercise

Codex can add the next exercise while following the repository conventions.
State the topic and ask it to inspect the current progress before changing
anything:

```text
Create the next Rust learning exercise based on the current progress.
Teach Tokio `join!` with a short theory section, a worked example, a focused
task, and unit tests. Follow AGENTS.md exactly: create main.rs, an identical
TODO.rs, and README.md; register the binary in Cargo.toml; add Makefile run and
test targets; preserve all existing exercises; and verify the scaffold.
```

For a backend-oriented exercise:

```text
Create a new exercise for graceful shutdown of a Tokio service.
Relate it to C# cancellation tokens and Go contexts. Include tests for normal
completion and shutdown, but do not put the final solution in README.md.
```

The generated exercise should remain a learning scaffold: `main.rs` contains
the task and tests, `TODO.rs` is the untouched reset copy, and `README.md`
contains the theory and instructions without the final implementation.

## Use Codex for implementation and review

After reading the exercise README and attempting the task, ask Codex to review
only the active `main.rs`:

```text
Review my implementation of Exercise 30.
Run the focused tests and rustfmt check. Find correctness issues first,
explain the ownership and async behavior, and do not edit the code yet.
```

When you are ready for a fix, make that explicit:

```text
Apply the smallest fix for the issue you found, preserve TODO.rs, then run
the focused tests and formatting check.
```

Codex can also help interpret failures:

```text
The test fails with this output: ...
Explain the root cause, identify the relevant Rust rule, and suggest the next
change without rewriting the whole function.
```

Always check the resulting diff and run the relevant Cargo or Makefile target.
For the full repository test suite:

```bash
make test
```

For one exercise:

```bash
make 02_concurrency_09_tokio_cancellation
```

Codex is most useful as a learning partner when you control the implementation
and ask it to explain compiler feedback, review trade-offs, and verify focused
changes rather than blindly replacing the whole solution.
