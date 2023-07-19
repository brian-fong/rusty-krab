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


### Chapter 2 - A Guessing Game
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

### Chapter 7 - Managing Growing Projects
In Rust, the *module system* refers to the organization of our code,
specifying which details are exposed, which are private, and what
names are in each scope of our code.

The module system consists of:
- *Packages*: A Cargo feature to help build, test, and share crates.
- *Crates*: A tree of modules that produce a library or executable.
- *Modules*: Lets us control the organization, scope, and privacy of
  paths.
- *Paths*: A way of naming an item, such as a struct, function, or
  module.

Crates come in 2 forms: *binary* and *library*.
- Binary crates are programs that can be compiled to an executable,
  such as a command-line program or a server. Each binary crate must
  have a `main()` function explicitly defined.
- Library crates are programs that are meant to provide functionality
  and be shared with multiple projects. The `rand` crate is a popular
  library that provides random number generation.

The *crate root* is a source file that the Rust compiler starts from
and creates the root module of our crate.
- For a binary crate, the crate root file is usually `src/main.rs`;
  whereas for a library crate, the crate root is usually `src/lib.rs`.

A *package* is a bundle of crates that provides a set of
functionality. Each package contains a `Cargo.toml` file describing
how to build the crates included.
- Cargo is actually a package containing the binary crate that is
  executed when we use `cargo` commands in our terminal.

In the crate root file, we use the `mod` keyword to declare new
modules. We may use the `use` keyword to create shortcuts for
importable items (constants, structs, functions).

KEY: To import modules and submodules from different directories, we
need to create a `mod.rs` file which serves as the "crate root file"
for that given module's directory. Within this `mod.rs` file, we
define the submodules within the directory.

### Chapter 4 - Understanding Ownership
Ownership is Rust's most unique feature.
- Promotes memory saftey guarantees without a garbage collector.

Ownership Rules:
- Each value in Rust has a variable that's called its *owner*.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

Note: In Rust, a `String` is allocated on the heap and is able to
accommodate indefinite space. We may create a `String` from a string
literal using the `from` function
```rust
let s = String::from("hello");
```

String literals are fast and efficient since the contents are known at
compile time and the text may be hardcoded directly into the final
executable.

With the `String` type, in order to support a mutable, growable piece
of text, the memory must be requested from the operating system at
runtime and we also need a way of returning said memory to the
operating system when we no longer need it.

In languages with a *garbage collector* (GC), the GC keeps track and
cleans up memory automatically for us. However, without a GC, it's the
programmer's responsiblity to identify when memory is no longer needed
and explicitly call code to return said memroy.
- Forgetting to return unused memory results in wasting memory.
- Returning memory too early results in an invalid variable.

We need to pair each `allocate` with exactly one `free`.

In Rust, there is a natural point where we can return the memory back
to the operating system. This point is when the variable associated
with said memory goes out of scope.

The `drop()` function is responsible for returning memory.
- Rust calls `drop()` automatically at the end of a scope, often
  denoted by "}".

In summary, the ownership of a variable is transferred when its value
is assigned to another variable. When a variable that includes data on
the heap goes out of scope, the value will be cleaned up by `drop()`,
unless ownership has been moved to another variable.

Handling ownership properly in Rust may be considered tedious if we
want to be able to continue to use variables after passing them into a
function i.e. prevent a function from taking over ownership. This
requires us to pass back all arguments that are passed in, along with
the data associated with the function's return.

Rust provides a feature to address the issue above called
*references*.
- References in Rust are denoted by the ampersand symbol "&".
- A reference is similar to a pointer in that it provides an address
  at which the data is stored; said data is owned by another variable.
  However, a reference is guaranteed to point to a valid value
  throughout its entire life, unlike a pointer.

We use `&mut` to declare *mutable references*.
- Mutable references have one large restriction: there may only be 1
  mutable reference for a given piece of data in a given scope. 
- This restriction prevents data races at compile time. A *data race*
  refers to concurrent operations attempting to access the same memory
  location with at least one of the operations being a write operation.
  These concurrent operations with the lack of explicit synchronization
  results in unpredictable and inconsistent results.
- There is also a similar restriction when creating mutable and
  immutable references to the same value.
- In other words, multiple immutable references are allowed because
  these operations don't involve write operations; the order of
  multiple read operations is harmless.

Note: The `println!()` macro, along with the other popular macros
associated with printing to the screen, do NOT take ownership of the
arguments passed into it! These macros create silent references behind
the scenes for reasons of convenience.

Rust also prevents *dangling pointers*, which are pointers that
reference a location in memory that may have been allocated to another
pointer. This situation often happens by freeing up some memory while
still maintaining a pointer to said memory.

In Rust, *slices* are segments of data that consist of two values: a
pointer to the index of the original data and a length.

String literals are of type `&str`, which is an immutable reference.
Hence, string literals are immutable.

In a sense, `&str` type is a superset of `String` type, since
`&String` can be sliced, turning it into a `&str`.

### Chapter 5 - Using Structs
A *struct* is a custom data type in Rust that requires us to specify
named values that collectively make up a meaningful group.
- Similar to object's data attributes in object-oriented languages.

In addition to structs, Rust also provides *tuple structs*, which are
similar but do not have names associated with their fields. Rather,
the fields of a tuple struct specifies the types of the fields.

### Chapter 7 - Packages, Crates, and Modules

