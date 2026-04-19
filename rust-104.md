### Module system:
Module system includes

- Packages: A Cargo feature to build, test, and share crates
- Crates: A tree of modules that produces a library or binary executable
- Modules and use: Let you control the organization, scope, and privacy of paths
- Paths: A way of naming an item, such as a struct, function, or module

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

### Control Scope & Privacy with module (`paths`, `use`, `pub`, `as`):
- paths - allows to use name items
- use - keyword that bring path into scope
- pub - keyword that make something public
- as - casting
- mod - for defining a module or module tree (containing other modules)
- external package -
- glob operator


### Module System:
Module are used to group related definitions together. Codes can be navigated based on the groups (rather than reading unrelated codes).

Modules are defined by `mod` keyword followed by the name of the module.

* private & public: In Rust, all items (functions, methods, structs, enums, modules, and constants) are private to parent modules by default. Use `pub` keyword to make anything public.

* Items in a parent module can’t use the private items inside child modules, but items in child modules can use the items in their ancestor modules. This is because child modules wrap and hide their implementation details, but the child modules can see the context in which they’re defined.