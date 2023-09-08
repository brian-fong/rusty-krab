# Zero to Production in Rust

### Chapter 1 - Getting Started

The Rust Toolchain
- To install Rust on a system, it is recommended to use `rustup`.
    - `rustup` serves as a *toolchain manager*. A *toolchain* is the
    combination of a *compilation target* and a *release channel*.

- Given that the Rust compiler's main purpose is to convert Rust code
into machine code (instructions that a CPU and OS can execute), there
is a need for specific Rust compiler backends for each compilation
target. In the end, we want to produce a runnable executable for each
platform (e.g. 64-bit Linux or 64-bit OSX).
    - The Rust project organizes the different updates and versions of
    Rust into 3 main release channels: *stable*, *beta*, and
    *nightly*.
    - `rustup` provides out of the box the latest stable compiler with
    the host platform (i.e. our machine) as the compilation target.
    - `rustup toolchain list` provides an overview of what is
    installed on our system.
    - There will be no need for cross-compiling from our development
    machine to the target host used in the production environment
    since our production workload will be containerized. 

Project Setup
- Use `cargo` to create a new Rust project
- `rust-analyzer` is an implementation of the Language Server Protocol
for Rust.

The *inner development loop* refers to a process whereby for every
significant change to our source code, we compile the application, run
tests, and run the application.
- For Rust code, compilation speed can become a pain point for big
projects.
- A significant chunk of compilation time is spent on the *linking
phase*, which assembles the actual binary given the outputs of the
earlier compilation stages.

Faster Development Experience
- There are alternative linkers to the default linker provided by the
Rust compiler.
    - `lld` on Windows and Linux
    - `zld` on MacOS
- Note: as of rust 1.70, `lld` is the default linker

- We may also use `cargo-watch` to reduce the perceived compilation
time.
    - To install, run `cargo install cargo-watch`
    - `cargo-watch` monitors our source code and triggers commands
    every time a file changes.

Continuous Integration
- In trunk-based development we should be able to successfully deploy
our `main` branch at any point in time. Every team member can branch
off from `main`, develop a small feature or fix a bug, merge back into
main, and release to our users.
    - Reduces the likelihood of merge conflicts due to long-lived
    branches.
- A *CI pipeline* refers to a set of automated checks that run on
every commit.
- We may leverage `cargo` to build our project and run
unit/integration tests via `cargo test`.
- CI includes tests, code coverage, linting, formatting, and checking for
security vulnerabilities.
    - [Tarpualin](https://github.com/xd009642/tarpaulin) is a code
    coverage reporting tool for the Cargo build system (named for the
    waterproof cloth used to cover cargo on a ship).
    - [clippy](https://github.com/rust-lang/rust-clippy) is the
    official Rust linter.
    - [rustfmt](https://github.com/rust-lang/rustfmt) is the official
      Rust formatter.
    - [`cargo audit`](https://docs.rs/cargo-audit/latest/cargo_audit/) is a
    useful cargo subcommand that checks for reported vulnerabilities for any
    crates within our project's dependency tree.
- For our CI pipeline, we would like to reject a commit if `clippy` emits
  any warnings or if `rustfmt` detects unformatted code.
