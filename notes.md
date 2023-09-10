# Zero to Production in Rust
by Luca Palmieri


## Chapter 1 - Getting Started
---
### The Rust Toolchain
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

### Project Setup
- Use `cargo` to create a new Rust project.
- Note: `rust-analyzer` is an implementation of the Language Server
Protocol for Rust.

### Development Loop
- The *inner development loop* refers to a process whereby for every
significant change to our source code, we compile the application, run
tests, and run the application.
- For Rust code, compilation speed can become a pain point for big
projects.
- A significant chunk of compilation time is spent on the *linking
phase*, which assembles the actual binary given the outputs of the
earlier compilation stages.
    - There are alternative linkers to the default linker provided by
    the Rust compiler.
        - `lld` on Windows and Linux, and `zld` on MacOS
    - Note: as of rust 1.70, `lld` is the default linker
- We may also use `cargo-watch` to reduce the perceived compilation
time.
    - To install, run `cargo install cargo-watch`
    - `cargo-watch` monitors our source code and triggers commands
    every time a file changes.

### Continuous Integration
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


## Chapter 2 - Building an Email Newsletter
---
> *Choose a problem you want to solve. Let the problem drive the
> introduction of new concepts and techniques. It flips the hierarchy
> you are used to.*

### User Stories
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

## Chapter 3 - Sign Up a New Subscriber
---
### Web Framework
- Article: [Choosing a Rust web framework, 2020
edition](https://www.lpalmieri.com/posts/2020-07-04-choosing-a-rust-web-framework-2020-edition/)
- `actix-web` will be our go-to web framework.

### Example: Hello World!
Let's start with a basic hello-world example from the `actix-web`
website:
```rust
use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

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
- `HttpServer` is the backbone of our web application, it handles
concerns on the *transport level*. It takes care of things like: 
    1. Where should the application be listening for incoming requests?
    In this case, we're listening on a TCP socket set at
    `127.0.0.1:8000`.
    2. What is the max number of concurrent connections? How many new
    connections per unit of time?
    3. Should we enable transport layer security (TLS)?

- `App` is the object containing our application's logic: routing,
middlewares, request handlers, etc.
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

### `#[tokio::main]` Macro
Let's take a look at the `main` function:
```rust
#[tokio::main]
async fn main() -> std::io::Result<()> {
    ...
}
```
- What is `#[tokio::main]`, and what does it do? If we take out
`#[tokio::main]`, then we encounter an error stating that `main` is
not allowed to be `async`.
    - We need `main` to be `async` because `HttpServer::run` is an
    `async` method. However, the Rust compiler actually requires
    `main`, the entrypoint of our binary, to not be `async`. This
    requirement comes from the fact that asynchronous programming in
    Rust is built on top of the `Future` trait.
        - Note: a `future` refers to a value that may not be there
        yet. All `future` instances expose a `poll` method, which must
        be called in order for the given `future` instance to make
        progress and eventually resolve to its final value. We may
        consider `future` instances to be "lazy"; that is, unless
        polled, there is no guarantee that they will execute to
        completion (this design is described as a "pull model", as
        opposted to a "push model" present in other languages).
    - Additionally, Rust does not include an asynchronous runtime by
    design. In order to utilize an asynchronous runtime, we are
    required to import one into our project as a dependency (i.e. one
    more create under `[dependencies]` in our `Cargo.toml` file).
    - If `main` were to be an asynchronous function, then whose
    responsbility is it to call the `poll` method? Thus, the Rust
    compiler sets the expectation to launch an asynchronous runtime at
    the tope of the `main` function, and then use it to drive `future`
    instances to completion.

- `#[tokio::main]` is a procedural *macro* that executes the
    asynchronous `main` function within the actix runtime.
    - Run `cargo expand` subcommand to expand macros and understand
    what's going on behind the scenes.
    - Note: macros are a form of meta-programming, code that generates
    other code.

- The following is the result of *macro expansion*:
```rust
fn main() -> std::io::Result<()> {
    let body = async {
        HttpServer::new(|| {
                App::new().route("/checkhealth", web::get().to(check_health))
            })
            .bind("127.0.0.1:8000")?
            .run()
            .await
    };
    #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
```
- Looking at the above expansion, we can see that `#[tokio::main]`
starts up tokio's `async` runtime, which serves to drive the `future`
instance returned by `HttpServer::run` to completion.
    - In addition, we see that the output of `main` is actually
    wrapped in an `async` body.
    - It seems like the `#[tokio::main]` macro creates an asynchronous
    `main` function meanwhile, behind the scenes, `main` is actually a
    synchronous function (as it should be) and the async `body` of
    our `HttpServer` is passed into tokio's runtime.


### `/checkhealth`  Endpoint
- User story #1 involves creating a `POST` endpoint for
`/subscriptions` route, where users may subsribe to a newsletter to
receive email updates. However, before we start addressing the
usecases of our app, let's first create a simple `GET` endpoint
`/check_health`, which returns a `200` OK response with no body.

- Creating a new endpoint involves creating a request handler
(`check_health` function) and then registering the handler to the
appropriate route.
```rust
// main.rs
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn check_health() -> impl Responder {
    HttpResponse::Ok()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/checkhealth", web::get().to(check_health))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
```
- To test our newly-created endpoint, open two terminals: one to
execute `cargo run`, which starts our HTTP server, and the other to
execute `curl -i http://127.0.0.1:8000/checkhealth` command, which sends a GET request to our endpoint.

### Tests: Endpoints
- Earlier, we tested our `/checkhealth` endpoint by manually sending a
GET request via `curl`. These manual tests can be increasingly
time-consuming and inefficient as our app grows.

- What is an API?
    - An API can be described as a tool to the outside world to
    perform some task. The endpoints exposed in our API define the
    *contract* between us and our clients, a shared agreement about
    the inputs & outputs of the system—an *interface*.

- How should we test an API endpoint?
    - By interacting with it in the same way as users would,
    performing HTTP requests against it and verifying assumptions on
    the responses. This practice is called *black box testing*.

### Integration Tests
- `actix-web` does provide some convenient testing tools, but there
  can be severe shortcomings when using framework-specific tools.
    - Migrating to another web framework would require us to rewrite
    our whole integration test suite. Thus, we'd like to create
    integration tests that are highly decoupled from the underlying
    API technology.

### Organizing Tests in Rust
- Rust provides 3 options to write tests:
    1. Next to our code in an *embedded test module*.
    2. In an external `tests` folder.
    3. As part of our public documentation, *doc tests*.

- An embedded test module is part of our project, just hidden behind a
*configuration conditional check* (`cfg`).
    - Since embedded test modules live next to the code they are
    testing, these modules have exclusive access to structs, methods,
    fields, and functions that have not been marked public and would
    normally not be available to users who import the code as a
    dependency.
    - Embedded test modules are useful when API exposure is limited
    (i.e. a few public functions), but the underlying source code is
    much larger and more complex.
    - Embedded test modules are also useful for writing unit tests for
    private sub-components of the source code to help increase the
    overall correctness of the project.

- On the other hand, anything under the `tests` folder or
documentation tests are compiled in their own separate binaries, which
may involve handling some *visibility rules*.
    - These tests would have exactly the same level of access to our
    source code as users who would import our code as a crate. Hence,
    this approach is mostly used for *integration testing*.

- For our email newsletter API, we'll be using the `tests` folder.

### Refactor Project Structure for Testing
- Currently, our project is a *binary*, which is meant to be executed
and not shared/imported. Thus, in its current state, we cannot import
our `main` function into modules within a separate `tests` folder.

- We need to refactor our project into a *binary* and a *library*,
where all our logic lives in the library crate while the binary itself
will be an entrypoint with a minimal `main` function.
- Reference: [Cargo Targets](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#cargo-targets)

- Currently this is what our `Cargo.toml` file looks like:
```toml
[package]
name = "rusty-krab"
version = "0.1.0"
edition = "2021"

[dependencies]
...
```

- We'll add a `lib` (library) and `bin` (binary) section:
```toml
[package]
name = "rusty-krab"
version = "0.1.0"
edition = "2021"

[lib]
path = "src/lib.rs"

[[bin]]
path = 'src/main.rs'
name = "rusty-krab"

[dependencies]
...
```
- And also create a `lib.rs` file in our `src` directory.

- Now that we have a binary and a library in our project, we can move
our application logic to our library and import the library
function(s) within our binary's `main` function.

### Test: `/checkhealth`
- To test the `/checkhealth` endpoint, we'll need an HTTP client. For
  our usecase, we'll use [reqwest](https://docs.rs/reqwest/latest/reqwest/).
    - To install `reqwest`, we'll add it under the
    `[dev-dependencies]` section in our `Cargo.toml` file.
