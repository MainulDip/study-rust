### Module system:
Module system includes

- Packages: A Cargo feature to build, test, and share crates
- Crates: A tree of modules that produces a library or binary executable
- Modules and use: Let you control the organization, scope, and privacy of paths
- Paths: A way of naming an item, such as a struct, function, or module

* In Rust, `mod name;` (semicolon) declares an external module, telling the compiler to look for code in a separate file (e.g., name.rs or name/mod.rs), while `mod name { ... }` (curly braces) defines an inline module with its contents nested inside the braces. 


### Package and Crates:
A crate is the smallest amount of code (both source code & compiled artifact) that the rust compiler consider at a time. This code/artifact can contain multiple modules (struct, enum, etc) from multiple files. A crate represents a tree of modules compiled together, not individually.

Crate can be of 2 kinds
- binary (executable) crates (ie, cmd tools, server, etc)
- library (non-executable) crates (ie, rand crate from crates.io)

A package in Rust is a bundle of one or more crates that provides a specific functionality and is managed by Cargo. A package contain a `Cargo.toml` file to describe how to build those crates. A package can contain multiple binary crates, but only one library crate. A package must contain at least one crate, whether that’s a library or binary crate.

Cargo (rust package manager and build tool) by convention follows
- `src/main.rs` as the crate root of a binary crate
- `src/lib.rs` as the crate root of a library crate
- `src/bin` directory is for placing multiple separate binary crate


*  If a package contains src/main.rs and src/lib.rs, it has two crates: a binary and a library, both with the same name as the package. A package can have multiple binary crates by placing files in the src/bin directory: Each file will be a separate binary crate.

### Module cheat sheet:
- Start form `crate` root (`src/main.rs and src/lib.rs`)
- Module declaration: Declare module/s (parent) in crate root. 
    - ex: `mod garden;` is used to declare external module, here compiler will search for the module in
        - current file with `mod garden {...}` or
        - `src/garden.rs` file or
        - `src/garden/mod.rs` file
- SubModule: are declared any file other than crate root.
    - ex: `mod vegetables;` declaration tell compiler to search in
        - current file with `mod vegetables {...}` or
        - `src/garden/vegetables.rs` file or
        - `src/garden/vegetables/mod.rs` file

- Paths to code in module: If a module is part of the working crate, it can be referred directly using crate root by `crate::parent_module::sub_module::...` (if not private)

- Private vs Public: Everything is private by default. Denote with `pub` to make it public

- Create path shortcut by `use`: Within a scope, the use keyword creates shortcuts to items to reduce repetition of long paths. Ex, `use crate::garden::vegetables::Asparagus;` declaration at the top of the file let use call `Asparagus` directly


```rust
// src/main.rs file
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}

// ---- end of file src/main.rs

// src/garden.rs file
pub mod vegetables;

// ---- end of file src/garden.rs

// src/garden/vegetables.rs file
#[derive(Debug)]
pub struct Asparagus {}

// ---- end of file src/garden/vegetables.rs file
```

### Control Scope & Privacy with module (`paths`, `use`, `pub`, `as`):
- paths - allows to use name items

- use - keyword that bring path into scope, creates shortcut to reduce repetition of long paths

- pub - keyword that make something public

- as - casting

- mod - for defining a module or module tree (containing other modules)

- external package -

- glob operator


### Module System to group related code:
Module are used to group related definitions together. Codes can be navigated based on the groups (rather than reading unrelated codes).

Modules are defined by `mod` keyword followed by the name of the module.

* private & public: In Rust, all items (functions, methods, structs, enums, modules, and constants) are private to parent modules by default. Use `pub` keyword to make anything public.

* Items in a parent module can’t use the private items inside child modules, but items in child modules can use the items in their ancestor modules. This is because child modules wrap and hide their implementation details, but the child modules can see the context in which they’re defined.


```rust
///*
/// We define a module with the mod keyword followed by the name of the module (in this case, front_of_house). 
/// The body of the module then goes inside curly brackets. Inside modules, we can place other modules, as in this case with the modules hosting and serving. 
/// Modules can also hold definitions for other items, such as structs, enums, constants, traits, and as in Listing 7-1, functions.
///  */

mod front_of_house {
   pub mod hosting {
        pub fn add_to_waitlist() { println!("Hello from add_to_waitlist") }
        fn seat_at_table() { println!("Hello from seat_at_table") }
    }

    mod service {
        fn take_order() { println!("Hello from take_order") }
        fn server_order() { println!("Hello from server_order") }
        fn take_payment() { println!("Hello from take_payment") }
    }
}

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();
}
```


* when a package containing both binary and library crate (src/main.rs & src/lib.rs), other project can benefit by the library crate by using functionality from there.


