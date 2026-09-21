# Installation

This guide prepares the local environment for the Rust exercises in this
repository.

## Required tools

Install the following:

- [Rust](https://www.rust-lang.org/tools/install), including `rustup`, `rustc`,
  and Cargo
- `make`
- Git

### macOS

Install Rust from the [official Rust installation page](https://www.rust-lang.org/tools/install).
Install Git and Make with [Apple's Command Line Tools](https://developer.apple.com/xcode/resources/)
or [Homebrew](https://brew.sh/). Install
[Docker Desktop for Mac](https://docs.docker.com/desktop/setup/install/mac-install/)
if you need PostgreSQL integration tests.

### Windows

Install Rust from the [official Rust installation page](https://www.rust-lang.org/tools/install).
Install Git from the [official Git for Windows page](https://git-scm.com/download/win).
Install Make through [Microsoft's WSL documentation](https://learn.microsoft.com/windows/wsl/install)
or another trusted Windows development environment. Install
[Docker Desktop for Windows](https://docs.docker.com/desktop/setup/install/windows-install/)
if you need PostgreSQL integration tests.

### Debian or Ubuntu Linux

Install Rust from the [official Rust installation page](https://www.rust-lang.org/tools/install).
Install Git and Make with the
[official Ubuntu installation guide](https://ubuntu.com/tutorials/install-and-configure-ubuntu-desktop#3-installing-applications)
or your distribution's package manager. Install Docker Engine with the
[official Docker instructions](https://docs.docker.com/engine/install/), including
the Compose plugin, if you need PostgreSQL integration tests.

After installing Rust, use the stable toolchain and verify the core commands:

```bash
rustup default stable
rustc --version
cargo --version
make --version
git --version
```

Rust formatting is used by the exercises. Install the component if it is not
already available:

```bash
rustup component add rustfmt
```

## Optional database test infrastructure

Some PostgreSQL integration exercises use Docker Compose to start a temporary
database. Install [Docker Desktop](https://docs.docker.com/desktop/) on macOS
or Windows, or Docker Engine with the Compose plugin on Linux.

Verify Docker Compose before running infrastructure-backed tests:

```bash
docker --version
docker compose version
```

The regular exercises do not require a running PostgreSQL server. The
repository's infrastructure Make targets start PostgreSQL on demand and clean
it up after the tests.

## Verify the repository

From the repository root, download dependencies and check the registered
exercise binaries:

```bash
cargo check
```

Run the exercise test suite with:

```bash
make test
```

For the full workflow, see [`README.md`](README.md) and
[`CODEX.md`](CODEX.md).
