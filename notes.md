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
Rust adopts a *module system* that includes the following:
- *Packages*: A Cargo feature that promotes building, testing, and
  sharing crates.
- *Crates*: A tree of modules that produces a library or executable.
- *Paths*: A way of naming an item such as a struct, function, or
  module.

A *crate* is any amount of code that the Rust compiler processes.
- Crates come in two forms: *binary* or *library*.
- A *binary crate* defines a `main()` function and is compiled into an
  executable, such as a CLI program or a server.
- A *library crate* defines structs, functions, and more to be shared
  with multiple projects. There is no `main()` function and does not
  compile to an executable.

A *package* is a bundle of crates that provides some functionality.
- Each package should contain a `Cargo.toml` file to describe how to
  build the crates within the package.
- A package may contain multiple binary crates, but at most only one
  library crate.

The `use` keyword creates *paths*, or shortcuts, to items defined
within other crates. These shortcuts reduce repitition of long module
paths.

The `crate root` is a source file that the Rust compiler starts
from and makes up the root module of your crate.
- In binary crates, the crate root is `src/main.rs` and in library
  crates, the crate root is `src/lib.rs`.
- `main.rs` and `lib.rs` formulate the module named `crate` at the
  root of our *module tree*, which scopes out the structure of our
  modules.
- The module tree can be thought of as the filesystem in our computer.

In Rust, all items (functions, methods, structs, enums, modules, and
constants) are private by default. We use the `pub` keyword to make an
item public and accessible from another file.

### Chapter 8 - Common Collections
*Collections* are data structures used to store multiple values.
Unlike the built-in `array` and `tuple` types, collections are stored
on the heap, which means the amount of data may be unknown at compile
time, and may grow or shrink as the program runs. There are many
different types of collections:
- A *vector* stores a variable number of values next to each other.
- A *string* is a collection of characters (`String` type).
- A *hash map* allows us to associate a value with a particular key.

Type annotations are necessary in Rust when the compiler is unable to
infer what type the value should be. In most cases, we would be
specifying an initial value of some sort, from which the Rust compiler
may infer the variable type. However, in the absence of an initial
value, we must apply explicit type annotation.
- Note: type inference may also occur when we mutate the variable by
  assigning a value of a certain type to it.

Rust allows mutable and immutable iteration over a vector. The
reference to the vector in the for-loop works to prevent simultaneous
modification of the whole vector. That is, we may change individual
elements of the vector without issue, but we will encounter a compiler
error when we attempt to insert or remove items from the whole vector.

Vectors are data structures that may contain only one type. Enums work
hand-in-hand with vectors in that enums provide variants and each enum
variant is still considered to be the same enum type. Hence, Rust
allows vectors to store values of different types as long as they all
are variants of the same enum type.

Note: when a vector variable goes out of scope, the vector as well as
its contents are dropped. Moreover, the borrow checker ensures that
any references to the contents of the vector are only used while the
vector itself is valid.

In Rust, strings are implemented as a collection of bytes accompanied
with some methods to provide useful functionality in the context of
interpreting these bytes as text.

Note: The `str` type is the only string type in Rust's core language.
The `String` type is provided by the standard library. Both `str` and
`String` types are UTF-8 encoded.

The `String` class is actually implemented as a wrapper around a
vector of bytes with additional guarantees, restrictions, and
functionality.
- In other words, `String` type is a superset of `Vec<T>`.

Rust strings do NOT support indexing like in most other programming
languages. The reason behind this design is due to the fact that all
strings are encoded in UTF-8, and some Unicode characters occupy 1 or
2 bytes of storage. This variation results in potentially invalid or
unexpected outcomes such as returning the byte value for an index
that is part of a two-byte character.
- Hence, Rust asks us to be more specific by defining a range of
  indices to create a string slice containing specific bytes.
- Warning: even specifying a range of indices may lead to
  conflicts, particularly when we cut off a multi-byte character.

*Hash maps* are useful data structures when we wish to look up data
not by using an index but with a key of some type.
- Other programming languages implement hash maps but often use a
  different name such as hash, map, object, hash table, dictionary, or
  associative array.
- Hash maps are NOT included in the prelude; we must explicitly
  include hash maps into the scope by importing `use
  std::collections::HashMap`.
- Hash maps are stored on the heap.
- Hash maps must be homogeneous, meaning all keys must be of one type
  and all values must be of one type.
- Hash maps do not accept references because the values that these
  references point to must be valid for at least as long as the hash
  map is valid.

