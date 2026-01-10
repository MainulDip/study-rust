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

### Using Libraries (Carets) | version management with cargo | Cargo.toml or `cargo add <crate>`:

`Cargo.toml` (Tom's obvious minimal language) is the place for versioning. 
Inside, each section is divided by a header, like `[package]`, `[dependencies]` etc. 
`Cargo.lock` file is used internally by cargo to track dependencies versions, not for manual editing.

`cargo update` command will ignore the `Cargo.lock` file, and will update the latest minor version. IE, `0.7.1` will update to maximum of `0.7.9` if available, and also will notify for available latest major release. To update to major release, editing the `Cargo.toml` is the option to go.

* always run `cargo build` or `cargo run` to compile the packages

```toml
[package]
name = "guessing-game"
version = "0.1.0"
edition = "2024"

[dependencies]
rand = "0.8.5"
```

### Guessing Game:
A simple terminal game, the program will generate a random number between 1 & 100, and will ask the user to guess the exact number until it's correct.

```rust
// use std::cmp::Ordering;
// use std::io;
use std::{cmp::Ordering, io}; // importing libraries in a single line

use rand::Rng;
// importing a library is called `prelude` in rust. prelude = `an action or event serving as an introduction to something more important`
// std stands for `standard` library. `std::io` makes the input/output standard library available to the scope
// here, we're using `rand` library/caret's `Rng` trait
// The Rng trait defines methods that random number generators implement, and this trait must be in scope for us to use those methods.

fn main() {
    println!("Guess the number!");

    // creating the secret number
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret_number}");

    // println!("Please input your guess.");

    // The equal sign (=) tells Rust we want to bind something to the variable
    // let mut guess = String::new();

    /*
     * The `::` syntax in the `::new` line indicates that new is an associated function of the `String` type.
     * An associated function is a function that’s implemented on a type, in this case String.
     * You’ll find a new function on many types because it’s a common name for a function that makes a new value of some kind.
     * If we hadn’t imported the io module with use std::io; at the beginning of the program, we could still use the function by writing this function call as std::io::stdin.
     * The stdin function returns an instance of std::io::Stdin, which is a type that represents a handle to the standard input for your terminal.
     */

    // replace by same code inside of the loop block
    // io::stdin()
    // .read_line(&mut guess)
    // .expect("Failed to read line");

    // The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data
    // without needing to copy that data into memory multiple times.
    // like variables, references are immutable, so we need to specify `&mut` to mutate

    /*
     * read_line function returns a Result type. Result is an enumeration (Enum), often called an enum, which is a type that can be in one of multiple possible states. We call each possible state a variant.
     * Result’s variants are Ok and Err. The Ok variant indicates the operation was successful, and it contains the successfully generated value. The Err variant means the operation failed, and it contains information about how or why the operation failed.
     * Values of the Result type, like values of any type, have methods defined on them. An instance of Result has an expect method that you can call.
     * If this instance of Result is an Err value, expect will cause the program to crash and display the message that you passed as an argument to expect.
     * If the read_line method returns an Err, it would likely be the result of an error coming from the underlying operating system.
     * If this instance of Result is an Ok value, expect will take the return value that Ok is holding and return just that value to you so that you can use it.
     * Without expect, the program will compile, but issue warning showing the `error case is not handled`
     * In simple terms, expect is used to handle the error case
     */

    /*
    Convert the user input into number type u32
    - (Shadowing) creating a new variable of same name but different type (u32 here)
    - Shadowing is feature is often used when you want to convert a value from one type to another type
    - `trim` will remove any white-space or newline `\n` as user must press `return` to satisfy read_line function
    - The parse method on strings converts a string to another type
    - u32 for `positive number`
     */
    // replaced by match and error handling inside the loop
    // let guess: u32 = guess.trim().parse().expect("Please type a number");

    /*
    Comparing the Guess to the Secret Number
     */
    loop {
        let mut guess = String::new();
        println!("Please input your guess.");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
```

### Variable `let` and Constant `const`:
Variables `let variable_name` are immutable by default, to make it mutable, use `mut` keyword as `let mut variable_name`. Naming convention is `snake_case`

Constants are always immutable and cannot be evaluated during runtime or based on another variable. `const CONSTANT_NAME` to define a constant. Naming Convention is `SCREAMING_SNAKE_CASE`

### Rust naming convention: 
- Local Variables & Function Parameters: Use snake_case (all lowercase, words separated by underscores), e.g., let user_id = 1;, let file_name = "data";.

- Constants (const) & Statics (static): Use SCREAMING_SNAKE_CASE (all uppercase, words separated by underscores), e.g., const MAX_TIMEOUT: u32 = 30;.

- Struct Fields: Use snake_case.

- Types, Structs, Enums, Traits: Use UpperCamelCase (PascalCase), e.g., struct 

- UserAccount;, enum Direction { North, South }.

- Enum Variants: Use UpperCamelCase, e.g., MyOption::SomeValue.

- Functions & Methods: Use snake_case, e.g., fn calculate_total() {}.

- Modules: Use snake_case.

- Crates: Use snake_case
