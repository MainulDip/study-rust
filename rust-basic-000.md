### Cargo:
Creating project : `cargo init` or `cargo new <project name>`. also run `cargo --help` or `cargo help init` for more option.

Build and run at the same time : `cargo run`
Build only: `cargo build` and then navigate to `target/debug` dir and run `./<package_name>`

`cargo check` only to quickly check if there are any compiler errors, 

Building for release: `cargo build --release`


Ongoing : Guessing game https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

### String interpolation:
Rust use macro like `std::fmt` for format string.

```rust
let name = "Alice";
let age = 30;
let s = format!("{} is {} years old.", name, age);
// Result: "Alice is 30 years old."

let s2 = format!("{n} is {a} years old.", n = name, a = age); // variables can be injected with `named-argument`
let s3 = format!("{name} is {age} years old."); // variable can be put inside of the braces

// Expression should not be inside of the {}, ie, method call or complex computation
let width = 10;
let height = 5;
println!("Area: {}", width * height); // result of evaluating an expression should be put outside of the braces

// Raw String 
let world = "lovely world";
println!(r#"hello "{world}""#); // Output: hello "lovely world"
```

### Using Libraries:
