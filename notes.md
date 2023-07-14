# Learn Rust

### Chapter 1 - Introduction
Installing Rust for development on Arch Linux:
```bash
sudo pacman -S rustup   # Download toolchain manager
rustup default stable   # Download stable toolchain
```

Check if Rust is successfully installed:
```bash
rustc --version
```

The `main()` function is a special function in Rust programs in that
it is the first thing that runs.

`println!()` is a *macro* to print text to the screen. Macros are
denoted by the "!".

To run a Rust program, it must be compiled first. We can use `rustc`
to compile a Rust file (.rs), and then use `./` (on Linux) to run the
resulting executable.
```bash
rustc main.rs   # compile source code
./main          # run executable file
```

*Cargo* is the official build system and package manager for Rust.

To create a new Rust project:
```bash
cargo new <project_name>
```

Note: by convention, source code (the code that we write) goes into the `src` directory. This convention makes it so that the top-level directory of our project contains only README's, license information, configuration files, and pretty much anything else unrelated to the code.

`cargo build` - Compiles a Rust project
`cargo run` - Compiles and runs a Rust project

Note: `cargo build` creates an executable file in the `target/debug`
directory, whereas `rustc <file>` creates an executable within the
same directory.

`cargo build --release` - Builds a Rust project with optimizations, resulting in faster run time at the cost of a longer compile time.
- By default, `cargo build` builds our project without these
optimizations for the sake of faster re-compiling and re-building.


### Chapter 2 - Guessing Game
In Rust, variables are immutable by default.

Creating a mutable string variable:
```rust
let mut foo = String::new();
```
`String::new()` represents a function call that returns a new instance
of a `String` type.
- The `String` type is a growable, UTF-8 encoded bit of text

A `Result` instant is an enumeration, or an *enum*, which is a type
that can be in one of multiple possible states, each state being
considered a *variant*.
- `Result`'s variants are `Ok` and `Err`.
- `Result` has a method `expect()`, which crashes the program if
  `Result` is an `Err` or returns the value if `Result` is an `Ok`.

### Chapter 3 - Common Programming Concepts
The `let` keyword is used to define variables.
- Variables are immutable by default. We use the `mut` keyword to make
  a variable mutable.

The `const` keyword is used to define constants.
- Constants are absolutely immutable.

The concept of *shadowing* in Rust refers to the act of declaring a
new variable with the same name as a previous variable.
- The `let` keyword is used to shadow an existing variable.
- Shadowing is different than re-assigning a variable. By using
  `let`, we are able to mutate both the variable's value and type.
  Moreover, these transformations exist only until the variable is
  shadowed again or the scope ends.

