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

    // relative path (Starting with a module name means that the path is relative)
    front_of_house::hosting::add_to_waitlist();
}
```


* when a package containing both binary and library crate (src/main.rs & src/lib.rs), other project can benefit by the library crate by using functionality from there, ie, `crate::module::fn_name()`

* when available, both `src/main.rs` and `src/lib.rs` are combined to create a module name `crate` at the root of the crate's module structure, known as module tree. Hence we can refer using `crate::module...`. Both crates will have the package name by default

*. when both `src/main.rs` binary crate and `src/lib.rs` library crate are available, The module tree should be defined in `src/lib.rs`. Then, any public items can be used in the binary crate by starting paths with the name of the package. The binary crate becomes a user of the library crate just like a completely external crate would use the library crate: It can only use the public API. 


### `super::` keyword with relative path:

```rust
// super with relative path: super can be used to call parent defined (outer scoped function) from inside a scoped module 
fn some_other_task() {
    println!("Do something now");
}

mod back_of_house {
    fn fix_incorrect_order() {
        super::some_other_task();
        cook_order();
    }

    fn cook_order() { println!("Cooking the food") }
}
```


### `pub` with Struct vs Enum (not same):
Making a `Struct` public won't make any of it's field/s public. We need to explicitly annotate those fields with `pub` to make public. Struct with one or more private fields needs associated function to create instance of it. 

But denoting an enum with `pub` keyword, will make all enum cases public.

```rust
// `pub` with struct
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast.
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like.
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal.
    // meal.seasonal_fruit = String::from("blueberries");
}
```

* `pub` with enum makes everything public

```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```


### shortcut with `use` and it's Scope with module `mod`:
Creating shortcut with `use` keyword and using from a module scope always involve getting the right scope.

We can use `super::` to bring the shortcut into our current scope when the use is defined in the parent module

```rust
// `use` keyword and scope
use front_of_house::hosting;

fn test_use() {
    hosting::add_to_waitlist(); // works because its the same scope as the `use` been defined
}

mod other_module {
use super::front_of_house::hosting::add_to_waitlist;
    fn test_use() {
        super::hosting::add_to_waitlist(); // works
        // hosting::add_to_waitlist() // will not work in this scope, have to navigate out of its scope
        add_to_waitlist(); // works, as we've brought the function into our scope using `use` keyword

    }
}
```


### Naming conflict, idiomatic (established convention) usages of `use` path and re-naming with `as`:

Sometimes we differentiate library brought module from local module by not creating pin-point shortcut. IE, `SomeBaseModule::Submodule` instead of `SomeBase_Module::Submodule::Exact_Function`. So we can call brought module by `Submodule::Exact_Function()` and local function by just `local_function()` directly. This is idiomatic practice.


* idiomatic `use`: Bringing the function’s parent module into scope with `use` means we have to specify the parent module when calling the function. Specifying the parent module when calling the function makes it clear that the function isn’t locally defined while still minimizing repetition of the full path.

```rust
// idiomatic way to bring the standard library’s HashMap struct into the scope of a binary crate

use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

* Naming conflict resolution: Rust doesn't allow bringing two modules with same name using `use`. Bringing the parent module may solve this conflict.

```rust
// Naming conflict resolution
// bringing two Result types into scope that have the same name but different parent modules
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}
```

* Naming conflict can also be resolved by assigning a new local name for the created short-cut module

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

### `pub use` to re-export name/s:
`pub` and `use` can be combined, so that code outside of the `use`'s scope can call that. By this, we're bringing an item into scope, at the same time making that item available for others to bring into their scope.

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;
// Now that this pub use has re-exported the hosting module from the root module, external code can use the path restaurant::hosting::add_to_waitlist() to call this

// Before this change, external code would have to call the add_to_waitlist function by using the path `restaurant::front_of_house::hosting::add_to_waitlist()`

// which also would have required the front_of_house module to be marked as `pub`.

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

### Mechanism of External Package usages:
When we add dependency to the `Cargo.toml`, that dependency can be referred by it's name as the crate root.

ie, if we add `rand = "0.8.5"` in the Cargo.toml, then we call `use rand::...` to access all the library features.

```rust
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
```

* Note: the code above, creates shortcut for `Rng` trait, which let us access the `gen_range()` function.

* Note: creating shortcut/importing a trait, makes it's public methods (or generic methods) available for the scope. Let's dive deeper while exploring trait.

```rust
// use rand::Rng; // we're only bringing the Rng trait's scope
// we can also create shortcut (& import) both `thread_rng` and `gen_range` separately, and use the shortcut, it's the same
use rand::{thread_rng, Rng};

fn main() {
    // here, we're directly accessing the `rand` crate's thread_rnd function and calling gen_range (which comes with the Rng trait, we imported/shortcut earlier)
    let secret_number = rand::thread_rng().gen_range(1..10);

    // here we're using the shortcut for `thread_rng`
    let secret_number_2 = thread_rng().gen_range(1..=100);

    // directly calling `Rng` trait's function, without using any `use` block to import
    let secret_number_3 = rand::Rng::gen_range(&mut rand::thread_rng(), 1..=100);

    // using `use rand::{thread_rng, Rng}`, but differently, starting from `Rng::`
    let secret_number = Rng::gen_range(&mut thread_rng(), 1..=100);
}
```
