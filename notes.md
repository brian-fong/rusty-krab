# Zero to Production in Rust

### Chapter 1 - Getting Started
The Rust Toolchain
- To install Rust on a system, it is recommended to use `rustup`.
`rustup` serves as a *toolchain manager*. A *toolchain* is the
combination of a *compilation target* and a *release channel*.
    - Given that the Rust compiler's main purpose is to convert Rust
    code into machine code (instructions that a CPU and OS can
    execute), there is a need for Rust compiler backends for each
    compilation target. In the end, we want to produce a runnable
    executable for each platform (e.g. 64-bit Linux or 64-bit OSX).
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

