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
- CI includes tests, code coverage, linting, formatting, and checking
for security vulnerabilities.
    - [Tarpualin](https://github.com/xd009642/tarpaulin) is a code
    coverage reporting tool for the Cargo build system (named for the
    waterproof cloth used to cover cargo on a ship).
    - [clippy](https://github.com/rust-lang/rust-clippy) is the
    official Rust linter.
    - [rustfmt](https://github.com/rust-lang/rustfmt) is the official
    Rust formatter.
    - [`cargo audit`](https://docs.rs/cargo-audit/latest/cargo_audit/)
    is a useful cargo subcommand that checks for reported
    vulnerabilities for any crates within our project's dependency
    tree.
- For our CI pipeline, we would like to reject a commit if `clippy`
emits any warnings or if `rustfmt` detects unformatted code.

### Chapter 2 - Building an Email Newsletter

> "Choose a problem you want to solve. Let the problem drive the
> introduction of new concepts and techniques. It flips the hierarchy
> you are used to."

Capturing Requirements
- We will leverage *user stories* to scope out what kind of service(s)
this email newsletter provides. User stories take on the following
format
- "As a ..., I want to ..., so that ..."
    1. "As a blog visitor, I want to subscribe to the newsletter, so
    that I can receive email updates when new content is published on
    the blog".
    2. "As the blog author, I want to send an email to all my
    subscribers, so that I can notify them when new content is
    published."
    3. "As a subscriber, I want to be able to unsubscribe from the
    newsletter, so that I can stop receiving email updates from the
    blog."

Web Framework
- Article: [Choosing a Rust web framework, 2020
edition](https://www.lpalmieri.com/posts/2020-07-04-choosing-a-rust-web-framework-2020-edition/)
- `actix-web` will be our go-to web framework.

Basic Health Check
- User story #1 involves creating a `POST` endpoint for
`/subscriptions` route, where users may subsribe to a newsletter to
receive email updates.
- However, before we start addressing the usecases of our app, let's
first create a simple `GET` endpoint `/check_health`, which returns a
`200` OK response with no body.

Starting Point: Hello World!

Consider the following hello-world example:
```rust
#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
```
- `App` is the object that's handling incoming requests and provides
some response.
    - `App` implements the *builder pattern* where `new()` provides a
      clean slate to which we may add certain properties to. In
    addition, `App`'s API is considered *fluent*, which allows us to
    chain method calls one after another, each method setting some
    property while returning the `App` object itself.
- The `route` method is the simplest way to create endpoints for our
web server.
    - The `route` method takes in two parameters:
        1. `path`: a string, possibly templated e.g. `/{name}`.
        2. `route`: an instance of the `Route` struct.
- The `Route` struct combines a *handler* with *guards*.
    - Guards specify conditions that an incoming request must satisfy
      in order to be "matched" to a certain handler.
    - Guards implement the `Guard` trait (`Guard::check`).
    - `web::get()` is a short-cut for
    `Route::new().guard(guard::Get())`, which sets the handler to
    respond if and only if the incoming HTTP request is `GET`.
- In summary, `App` iterates over all registered routes until it finds
a matching one (both path template and guards are satisfied), and
invokes the corresponding handler, passing over the request object.
- `greet` is an asynchronous function that takes in an `HttpRequest`
and returns something that implements the `Responder` trait.
    - An object can implement the `Responder` trait if it can be
    converted into a `HttpResponse`. This trait is implemented off the
    shelf with various common types such as strings, status codes,
    bytes, etc.
    - Note: not all handlers must have the same function signature as
      `greet`. `actix-web` allows a wide range of function signatures
      for handlers!

Now let's consider the `main` function.
```rust
#[tokio::main]
async fn main() -> std::io::Result<()> {
    ...
}
```
- `#[tokio::main]` is a *macro* that executes the asynchronous `main`
  function within the actix runtime.
    - Run `cargo expand` subcommand to expand macros and understand
    what's going on behind the scenes.
    - Note: macros are a form of meta-programming, code that generates
      other code.
