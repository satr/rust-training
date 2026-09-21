# Using GitHub Copilot for Rust learning

GitHub Copilot can provide inline Rust completions, explain code, review
changes, and use an agent to work across this repository. It is especially
convenient in VS Code and on GitHub.

See the [official repository-instructions documentation](https://docs.github.com/en/copilot/how-tos/copilot-on-github/customize-copilot/add-custom-instructions/add-repository-instructions)
for current Copilot instruction-file support.

## Repository instructions

Copilot supports repository-wide `.github/copilot-instructions.md`,
path-specific `.github/instructions/*.instructions.md`, and agent-oriented
`AGENTS.md` files. The root [`AGENTS.md`](../AGENTS.md) is the source of truth for
this training repository.

If Copilot does not appear to use the instructions, attach the repository to
the chat and explicitly say:

```text
Read and follow the repository's AGENTS.md before answering. This is a Rust
training repository: preserve TODO.rs and teach before implementing.
```

## Ask for explanations

Use Copilot Chat or inline chat with focused questions:

```text
Explain this borrow-checker error in plain language. Relate it to C# and Go,
show a tiny example, and do not modify the exercise.
```

For an exercise review:

```text
Review only exercises/02_concurrency/08_tokio_channels/main.rs.
Check ownership, channel closure, error handling, and tests. Do not edit the
file and do not reveal the complete solution.
```

## Generate an exercise

For Copilot Chat or the Copilot coding agent:

```text
Read AGENTS.md, README.md, and the current progress log. Create the next
exercise in the correct numbered group. Include main.rs, identical TODO.rs,
README.md, unit tests, Cargo.toml registration, and a group-prefixed Makefile
target. Preserve existing exercises and do not put the final solution in the
README. Run the focused test and formatting checks.
```

## Training workflow

Prefer inline completion for small code fragments, then ask Copilot to explain
the generated code. Keep the implementation under your control:

```text
Give me one hint for this TODO and explain the relevant Rust rule. Do not
write the function for me.
```

Run the repository checks yourself or ask the agent to run them:

```bash
make 01_base_01_measurement
make test
```

Review the diff after agent edits and confirm that `TODO.rs` remains untouched.
