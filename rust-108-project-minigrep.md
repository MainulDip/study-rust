### Project initialization:
`cargo init` creates project in the current directory with no name supplied
`cargo init --name project_name` creates in current directory with the supplied name as project name
`cargo new <project_name>` creates project in a new directory as the project_name

* cargo init vs new: The primary difference between cargo init and cargo new is where the project files are created.


### Safely access vector element:
When you access a vector index that doesn't exist, the program will panic if not dealt.

- `v[index]` need to be absolute certain about the existence
- `v.get(index)` will return `Option<&T>`, so need to handle Some and None cases through match or use `unwrap_or(&0)`

- using `v.get_mut(index)` to mutably borrowing and changing an element

- using `v.get(index).unwrap_or(&default_value)` getting a default value if the index is missing

```rust
fn main() {
    let mut v = vec![10, 20, 30];

    // Immutably borrowing an element
    match v.get(1) {
        Some(value) => println!("Found: {value}"),
        None => println!("Index out of bounds"),
    }

    // Mutably borrowing and changing an element
    if let Some(value) = v.get_mut(1) {
        *value = 25; 
    }

    // Getting a default value if the index is missing
    let value = v.get(5).unwrap_or(&0);
    println!("Value or default: {value}");
}

```


### Single Reference vs Double Reference (&String vs &&String):
A &String is a borrowed reference to that text. A &&String is a reference to a reference. This double reference are usually seen (`&&String`) when a method or iterator yields references automatically, but you rarely need to write it yourself.


- &String (Single Reference)
    - Borrows the String without taking ownership.
    - Read-only by default.
    - Lets multiple parts of your code look at the text without copying it.

- &&String (Double Reference)
    - A reference that points to a &String box.
    - Happens during loops or iterator methods like .iter() over a collection of references.
    - Rust usually uses auto-dereferencing to let you call normal methods on it without trouble.


### Struct::new vs Struct::build:
Many programmers expect the `new` function should never fail. But the `build` function can fail, so all error handling can reside there.


### Ok(()) type, Box<dyn Error> and `?` and `if let`:
When returning a `Ok(())`, the `()` means we're returning nothing but doing something as side effect. The `()` is a empty tuple signature.

The `Box<dyn Error>` is a trait object, a function returning this means the function will return a type that implements the Error trait, but we don’t have to specify what particular type the return value will be. This gives us flexibility to return error values that may be of different types in different error cases. The dyn keyword is short for dynamic.

And the `?` operator is used with functions that can panic!. Rather than panic! on an error, ? will return the error value from the current function for the caller to handle.


The `if let Variable(error)` pattern works kinda same as unwrap_or_else to check is a function returns an `Error` value ignoring any `Ok` value, when the `Ok(())` is returning nothing but doing side effect.

```rust
use std::error::Error;

// --snip--

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}

fn main() {
    // --snip--

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
```

### `unimplemented!()` vs `todo!()` for unfinished code and some other options:
Both of these macros are same (will panic) but differ slightly for intentional purpose and both satisfy the compiler's type checking while prototyping.

- `unimplemented!("Optional Message")` is for Permanent/deliberate omission and ignored by IDE todo tracker. Emit message 'not implemented'. Support optional message.

- `todo!("Optional Message")` is for temporary/work-in-progress placeholder and are highlighted by IDE todo tracker, Emit message 'not yet implemented'. Support optional message.

There are some other helpful macros as well to deal with IDE and coding progression

- `unreachable!` : Panics instantly with "internal error: entered unreachable code". Used for default match arms where you have already handled all valid enums

```rust
match light {
    TrafficLight::Red => stop(),
    TrafficLight::Green => go(),
    _ => unreachable!("We only have Red and Green lights!"),
}
```

- `#[warn(clippy:todo)]` compiler attribute to flag incomplete work. This avoid runtime panics by forcing compile-time warnings or error through `cargo check` or `cargo build`. Best for keeping track of text-based reminders without breaking your program's execution.


```rust
#[warn(clippy::todo)]
fn finish_this_later() {
    // Clippy will trigger a compiler warning right here
}
```

- `compile_error!` to intentionally stop compilation before a binary is even created. Used mostly in conditional compilation (#[cfg]) to prevent unsupported platforms or feature flags from compiling.


```rust
#[cfg(not(target_os = "linux"))]
compile_error!("This crate only supports Linux operating systems.");
```

- `Option` and `Result` types: For production-safe prototyping, these can be used to avoid panicking macros entirely. Return an empty variant that caller functions can gracefully handle. These bubbles up a standard, safe error or non-value. Best used for public APIs where crashing the entire program is unacceptable.

```rust
fn get_user_v2() -> Option<User> {
    None // Placeholder until the database is wired up
}
```

### Splitting Code into Library Crate (alongside Binary Crate):
The `./src/main.rs` should do less work  and  should rely on library crate (`.src/lib.rs`) for business logics. That way, we can test the code (for both unit and integration test) and have the `.src/main.rs` file with fewer responsibilities. 

Defining all business logics inside lib crate will open more context for usages and possibilities for other people use the code.

```rust
// ./src/main.rs
use minigrep::search;
 fn main() {
    let content = fs::read_to_string(&file_name)?;

    for line in search(&config.query, &content) {
        println!("{line}");
    }
 }

// ./src/lib.rs
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    unimplemented!();
}
```

### TDD in rust (Test driven development):
Its the same workflow (consisting a loop of 4 steeps) as for other programming language.

1. Write a test that fails and run it to make sure it fails for the reason you expect.
2. Write or modify just enough code to make the new test pass.
3. Refactor the code you just added or changed and make sure the tests continue to pass.
4. Repeat from step 1!

```rust
use std::result;

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // unimplemented!();
    // first we'll return an empty vector to make the function to fail
    // vec![]
    // then we're gonna implement the function to pass the test. Do just as much required, no pre mature optimization
    let mut results: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }
}
```


### Environment variables:
In rust environment variable can be passed through compiler arguments or through `Cargo.toml` or directly inside code. They can be key-value pair or just a key (to check if a key exists or not).

The standard library provides `env::var` to look up a variable is something exists while the programme running. This function returns a Result enum (Ok(String) or Err(VarError)) because the requested variable might be missing or contain non-Unicode characters.

* reading for a environment variable can be done on both runtime or compile time

```rust
// sometime, we only need to be sure if some key was injected (maybe with `Key cargo run` command), not caring about the value (`KEY=VALUE cargo run)

fn main() {
    // maybe called by `IGNORE_CASE cargo run`
    let ignore_case_result: Result<String, VarError> = env::var("IGNORE_CASE").is_ok();
    let ignore_case: bool = ignore_case_result.is_ok();
    /*
    The env::var function returns a Result that will be the successful Ok variant that contains the value of the environment variable if the environment variable is set to any value. It will return the Err variant if the environment variable is not set.

    we're using is_ok rather than `unwrap` or `except`, as `is_ok()` will only return true or false if `IGNORE_CASE` was injected or not (to check a environment variable existence)
    */
}
```

* Setting environment variable for shell session and removing: `IGNORE_CASE=1 cargo run` will persist the environment for the shell session and `Remove-Item IGNORE_CASE` will remove that variable from the current shell session. For window this will be `$Env:IGNORE_CASE=1; cargo run` and `Remove-Item Env:IGNORE_CASE`

```rust
// using match and expect to get environment variable and handle the error case

use std::env;

fn main() {
    // Looks up the "DATABASE_URL" environment variable
    match env::var("DATABASE_URL") {
        Ok(url) => println!("Connecting to: {}", url),
        Err(e) => println!("Error or variable not set: {}", e),
    }

    // Shorthand if you just want to crash/panic if it is missing
    let api_key = env::var("API_KEY").expect("API_KEY must be set");
}

```

* Using `Cargo.toml` to host environment variables

```toml
[env]
# 1. Simple text variable
DATABASE_URL = "postgres://localhost/mydb"

# 2. Force override if the variable already exists on the system
API_KEY = { value = "secret_key", force = true }

# 3. Path relative to this config file (turns into an absolute path automatically)
LOG_FILE_PATH = { value = "logs/output.log", relative = true }
```

### `stdout` vs `stderr`:
`println!` macro is used for standard output (general information). For showing error message `stderr` rust provide `eprintln!` macro.

The stderr is helpful, when we're saving the program output to a file but still want to print error message in the terminal (if any error occurs). 


```rust
fn main() {
    // -- snip --
    if (program_runs_well) {
        // do the task
    } else {
        // program fails to run
        eprintln!("Application error");
    }
}

// cargo run -- to poem.txt > output.txt
// if some runtime error happens, the terminal will still print the error (because of eprintln!), without this the errors will go directly to the output.txt file (println!)
```