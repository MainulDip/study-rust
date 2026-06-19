### Common (heap-stored) Collection Type (other than built-in stack-stored array and tuple types):

- `vector: Vec<T>`: stores variable number of values next to each other
- `String`: collection of characters (from std library)
- `hash map`: allow to store value with key (key-value pair), `map` for other programming language.



### Vectors `Vec<T>`:
It's a heap stored collection that can store more than one value of the same type.

- Creating an empty vector `Vec::new()` and vector with initial values use `vec![1, 2, 3]` macro.

- check following code, watch out example for - vector creation, modification, iteration using for-in loop and borrower-checker restrictions.

```rust
use std::vec;

fn main() {
    println!(" Playing with vector 101 ------------------------- ");

    let mut empty_vector: Vec<i32> = Vec::new();
    // adding new elements by `push`
    empty_vector.push(1);
    empty_vector.push(2);
    empty_vector.push(3);

    let mut vector_with_initial_value = vec![1, 2, 3];
    vector_with_initial_value.push(4);

    let third: &i32 = &vector_with_initial_value[2]; // if index is out-of-bound, the program will panic and exit, here it isn't
    println!("The third element is {third}");

    let option_type_third = vector_with_initial_value.get(2);
    match option_type_third {
        Some(value) => println!("option_type_third is {value}"),
        None => println!("option_type_third doesn't exists"),
    }

    // vector iteration
    let mut vec_iter = vec![1, 2, 3];
    for i in &vec_iter { // here without `&` will also do the loop, but the ownership will be moved
        println!("i = {i} ");
    }
    println!("now the vec_iter = {:?}", vec_iter); // it works as we're using &vec_iter not `vec_iter` directly in the loop

    for i in &mut vec_iter {
        *i += 10; // we need to deference for further assignment in this case
        println!("i after transformation is = {i}");
    }
    println!("vec_iter still valid as we've not moved the ownership yet, proof = {:?}", vec_iter); // but it had been modified by the previous for-in loop to `[11, 12, 13]`

    // Same borrowing rules applied here, We cannot read an immutable borrowed property after it is also been mutably borrowed
    let mut some_vector = vec![1, 2, 3];
    let immutable_borrow_vec_element = &some_vector[2];
    some_vector.push(4);
    // println!("immutable_borrow_vec_element = {immutable_borrow_vec_element}"); 
    // will throw error, not possible to read after it had been modified, as borrowed rules applies
}
```


### Vector with Enum element type (store multiple types):
Other than using enums, we can't store lists of different types using vector. 

We can define an enum whose variants will hold the different value types, and all the enum variants will be considered the same type: that of the enum. Then, we can create a vector to hold that enum and so, ultimately, hold different types. 


```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

* another way to store multiple types in a collection is `trait object`, lets dive later on this.


* Like any other struct or variable, a vector is freed when it goes out-of-scope

```rust
{
    let v = vec![1, 2, 3, 4];

    // do stuff with v
} // <- v goes out of scope and is freed here
```

### String (Std) and Vector:
`String` is implemented as a wrapper around a vector of bytes with some extra guarantees, restrictions, and capabilities. 

```rust
// string creation is kinda same like vector with `String::new()` for empty string
let mut s = String::new();

// String from string-literal using `to_string`
let data = "initial contents";
let string_data = data.to_string();
let string_from_literal = "Initial contents".to_string(); // directly hooking with literal
let string_from_literal_2 = String::from("Initial contents"); // directly creation, but with `String::from` method
```

* Strings are `UTF-8` encoded, so we can include any properly encoded data inside

```rust
// any UTF-8 data will work with String type
let hello = String::from("السلام عليكم");
let hello = String::from("Dobrý den");
let hello = String::from("Hello");
let hello = String::from("שלום");
let hello = String::from("नमस्ते");
let hello = String::from("こんにちは");
let hello = String::from("안녕하세요");
let hello = String::from("你好");
let hello = String::from("Olá");
let hello = String::from("Здравствуйте");
let hello = String::from("Hola");
```

### String Operations:

* Concatenation: `push_str("string literal, aka slice")` for String with string-slice, and `+` for String with String join

* the `+` operator use the `add` method behind the scene, signature `fn add(self, s: &str) -> String`

```rust
let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2); // s1 is now "foobar"
// the s2 is string slice, so its ownership is not transferred, hence we can still access s2 later in the print statement below
println!("s2 is {s2}");


// String + String operation
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // s3 is now "Hello world!"
// note s1 has been moved here and can no longer be used/accessed. Because String is heap stored
```


### Deref Coercion (supplying `&String` where it accepts `&str`):
- Deref: Short for de-reference
- Coercion: To force, compel, or forcefully persuade someone to do something they do not want to do

Deref coercion is a language feature in Rust that automatically converts a reference to one type into a reference to another type. It happens when a type implements the `Deref` trait, allowing the compiler to treat smart pointers (like Box, String, or Vec) as regular references.

```rust
// though the `+` function signature is `fn add(self, &str) -> String`, as its accepting borrowed string-slice in its parameter, supplying a borrowed String will also work, as the compiler will forcefully convert &String into &str.

let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; 
// here, rust compiler is converting `&s2` into `&s2[..]` behind the scene
// note s1 has been moved here and can no longer be used, as the `fn add(self, &str)` signature is using owned self, it will be moved when used as heap stored String type (here it is)
// and, as expected, s2 is not moved, so can be use later.
```

### `format` macro:
Sometimes, concatenating with `+` can seem little bit untidy. Here comes the `format!` macro.

The `format!` macro works like `println!`, but instead of printing the output to the screen, it returns a String with the contents.

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

// using add `+`, which is little bit difficult to read
let s_add = s1 + "-" + &s2 + "-" + &s3;

// using format! macro, easier to read
let s_format = format!("{s1}-{s2}-{s3}");
```

### UTF-8 String in depth and why String indexing works differently:
Rust strings don't support indexing. 

Recap: Storing mechanism of UTF-8 (how unicode can store so many characters, more than a million, 1,114,112 to be precise)

    Unicode is like a giant list (like a phone book) where every character in the world is assigned a unique number called a Code Point, ranging from the letter "A" to the "Taco" emoji. 

    UTF-8: Unicode Transformation Format, it's a variable length of 8 to 32 bit (1 to 4 bytes). English characters usually take 1 byte, where more complex character (like emoji) can take all the way, up to 4 bytes (32 bits).

    UTF-16: It is also variable length, can be either 16bit or 32bit
    UTF-32: It's fixed length, 32bit (4bytes)


UTF-8 binary flags (little similar for utf-16):
As utf-8 can be 1 to 4 byte depending on the character, it uses first 1 to 5 bit as marker or flags

- 1 byte: If a byte starts with a 0, the computer knows: "This is a simple 1-byte character."
- 2 byte: If it starts with 110, it knows: "This is the start of a 2-byte sequence."
- 3 byte: If it starts with 1110, it’s a 3-byte sequence
- 4 byte: If it starts with `11110..`, it's a 4 byte sequence
- Continuation Header Flag: If a byte starts with `10..`, it tells the memory reader that "I'm not a new character, rather, a continuation of the previous character.


* why string indexing is not valid in rust:
- Rust's both String and String-slice are UTF-8 based. Though the String (STD) is a wrapper around vector (`Vec<u8>`), it doesn't support indexing operations, ie, `"Something"[0]` is not valid. Because, rust store strings in utf-8 encoded bytes and 1 bite doesn't represent a valid character always, as utf-8 can be 1 to 4 byte long.

* String iteration

```rust
for c in "কাক".as_bytes() {
    // println!("Byte value {c}");
    println!("{c}");
    // 224, 166, 149, 224, 166, 190, 224, 166, 149
}

for c in "কাক".chars() {
    println!("{c}");
}
// ক, া, ক
```

* Slicing String with range: can panic if the output slice is not a valid character (ie, part of a character, not full, see utf-8's variable byte length)

```rust
// Slicing, slicing should be done with caution, if the output/rendered byte is not a valid character, the program will panic
let s = String::from("কাক");
let string_slice_valid = &s[0..3];
println!("{string_slice_valid}"); // will print "ক", it's a valid character in bengali language

// let string_slice_not_valid = &s[0..1]; // will panic as the first byte is not a valid character, it needs 3 byte to form character "ক"
// println!("{string_slice_not_valid}"); // before reaching this point, the program will panic (to create a variable based on non character slice)
```


### HashMap and Hashing (Hashing Function) in general (HashTable DataStructure):
In general CS term, `hashing` is a mathematical formula to generate a fixed-size output from an input of variable size. Different programming language implement this differently. 

HashMap is the rust's implementation of general CS term `Hash-Table`, it has its own implementation of `hashing function` to generate unique fixed length identifier from the keys and storage mechanism as key-value pair.

### Hash Map (Key value pair, like map or dictionary or associative array):
Using HashMap data is accessed using key instead of index. It's defined in the rust standard library.

Hashmap is a key-value pair data structure (`HashMap<K, V>`) is common in other programming language, but they may  called different, ie, hash, map, object, hash table, dictionary, associative array, etc. 

HashMap's key is unique, and can have only one value associated (vector, tuple or Option<T> with Some(T) or None will also work).


* different ways to create hashmap with initial values

```rust
// using HashMap::from fn
let hash_map_all_numbe: HashMap<i32, i32> = HashMap::from([(10, 10)]); // funny (key as number), here it will accept list of tuple
let hash_map_str: HashMap<&str, i32> = HashMap::from([("a", 10)]);
let hash_map_string: HashMap<String, i32> = HashMap::from([(String::from("A"), 10)]);

// using collect with vector or iterator
let fruits_vector: Vec<(&str, i32)> = vec![("Apple", 10),("Orange", 20),("Mango", 30)];
// convert the vector into iterator and collect to make a hashmap
let fruits_hashmap: HashMap<&str, i32> = fruits_vector.into_iter().collect(); // into_inter() will work, but not iter(), as iter() will return a borrowed type

// merging 2 separate vector as hashmap using `zip()` and `collect()`
// when keys and values are in separate collections, .zip() fn can be used to pair them up into tuples and then call .collect() to create hashmap
let keys_vec = vec!["Apple", "Orange", "Mango"];
let values_vec = vec![10, 20, 30];

let zipped_fruits_hashmap: HashMap<&str, i32> = keys_vec.into_iter().zip(values_vec).collect(); // type needs to be explicit
```

* initializing HashMap with mutable option

```rust
let mut scores: HashMap<String, i32> = HashMap::new();
scores.insert(String::from("Cricket"), 370);
scores.insert(String::from("Football"), 4);
// when accessed by key using the `get` function, it returns an Option<T> (either Some(T) or None), so we need to unwrap that
println!("Scores for Cricket is {:?} and for Football is {:?}", scores.get(&String::from("Cricket")).expect("msg"), scores.get(&String::from("Football")).expect("nothing matches as the supplied key"));

// iterating over a HashMap
for (key, value) in scores {
    println!("Key {key} and value {value}");
}
```

### HashMap and Ownership (its the same principle):
For types that implement the Copy trait, like i32, the values are copied into the hash map. For owned values like String, the values will be moved and the hash map will be the owner of those values

```rust
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// field_name and field_value are invalid at this point, try using them and
// see what compiler error you get!
```

### Updating Hashmap (overwriting, pre-existing case, old value calculation):
Note: HashMap's key is unique, and can have only one value associated (vector, tuple or Option<T> with Some(T) or None will also work).

Updating HashMap can go different direction
- overwrite/replace the oldValue with newValue | `insert` with same key

```rust
users_balance.insert(userId, minimum_balance);
users_balance.insert(userId, new_balance);
```

- keep the old value and discard the new value | `entry(Key).or_insert(Value)`

```rust
let user_handle = users_balance.entry(userId).or_insert(0);
    if new_balance > minimum_balance {
        *user_handle = new_balance;
    }
```

- add the new value, if the key doesn't already have a value | `entry(Key).or_insert(Value)`

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{scores:?}");
```

- combine the old value and new value | `entry(Key).or_insert(Value)`

```rust
let mut hashmap: HashMap<&str, i32> = HashMap::new();
let text = "hello world wonderful world";

for word in text.split_whitespace() {
    let count = hashmap.entry(word).or_insert(0);
    *count += 1;
}

println!("hashmap = {:?}", hashmap);
```

* Complete the challenges: https://doc.rust-lang.org/book/ch08-03-hash-maps.html#summary

### Error Handling:
Rust error are 2 type:
- Recoverable Error (`Result<T,E>`): report error and retry, ie, file-not-found error
- UnRecoverable Error (`Panic!` macro): immediately stop the program, ie, accessing a location beyond the end of an array. These are usually introduced by some bug.

### UnRecoverable Error (`Panic!`):
Rust has the `Panic!` macro to deal with unrecoverable errors. `Panic!` can be manually triggered or automatically when bugs are introduced (ie, accessing an array or vector element past the end).

* By default, these panics will print a failure message, unwind, clean up the stack, and quit. Environment variables are also supported for stack trace and other features.

* By default, when a panic occurs, the program starts unwinding, which means Rust walks back up the stack and cleans up the data from each function it encounters. Which is expensive computation. `panic = 'abort'` can set in `Cargo.toml` to immediately exit without the cleanup.

```toml
// Cargo.toml
[profile.release]
panic = 'abort'
```

* now trigger a manual panic by calling the `panic!` macro with a message

```rust
// main.rs
fn main() {
    panic!("crash and burn");
}

// calling the `cargo run` will cause the program to panic with the provided message and some helpful information

// `RUST_BACKTRACE=all cargo run` will give backtrace info about the error. `Backtrace` is a list of all the functions that have been called to get to this point (error), The key to reading the backtrace is to start from the top and read until you see files you wrote.
```

* `Backtrace` is a list of all the functions that have been called to get to this point (error), The key to reading the backtrace is to start from the top and read until you see files you wrote. That’s the spot where the problem originated. The lines above that spot are code that your code has called; the lines below are code that called your code. These before-and-after lines might include core Rust code, standard library code, or crates that you’re using

* In order to get backtraces with this information, debug symbols must be enabled. Debug symbols are enabled by default when using cargo build or cargo run without the --release flag, as we have here.

### `Result<T, E>` enum for recovering from errors:
Result enum is defined as having two variants, `Ok` and `Err`

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

* We can use match case to handle error cases gracefully or panic if nothing works


```rust
use std::{
    fs::File,
    io::{Error, ErrorKind},
};

pub fn recover_error_one() {
    println!("Bismillah");
    get_file_and_report();
    get_file_and_report_with_generics();
}

// Error (`Result<T, E>`) handling with nested match 
fn get_file_and_report() {
    let greeting_file_result = File::open("hello.txt");

    // handling Result's cases for accessing the hello.txt file, whether is exists or not
    let greeting_file = match greeting_file_result {
        Ok(File) => File,
        // Err(Error) => panic!("File not found: {Error_Msg}"), // instead of panicking here, let's create the file if not exists already
        Err(Error) => match Error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(File) => File,
                Err(Create_Failed) => panic!("File cannot be created: {Create_Failed:?}"),
            },
            _ => panic!("File not found: {Error:?}"),
        },
    };

    println!("{greeting_file:?}");
}


// Result<T, E> handling with closure function defined inside of the Result
fn get_file_and_report_with_generics() {
    let file_name = "Hello2.txt";
    let greeting_file_result = File::open(file_name);

    // handle Result's case with closure (unwrap_or_else) instead of multiple nested match statement 
    let greeting_file = greeting_file_result.unwrap_or_else( |error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(file_name).unwrap_or_else(|err| {
                panic!("Problem creating the file {err:?}");
            })
        } else {
            panic!("Problem opening the file {error:?}");
        }
    });

    println!("{greeting_file:?}");
}


// Result<T, E> handling with unwrap method 
// If the Result value is the Ok variant, unwrap will return the value inside the Ok. If the Result is the Err variant, unwrap will call the panic! macro for us. 
// unwrap cannot have custom panic message
fn get_file_and_report_with_unwrap() {
    let file_name = "Hello3.txt";
    let greeting_file_result = File::open(file_name);

    let greeting_file = greeting_file_result.unwrap();
}

// Result<T, E> handling with `expect`
// In production-quality code, most Rustaceans choose expect rather than unwrap. As unwrap doesn't have custom message
fn get_file_and_report_with_expect() {
    let file_name = "Hello4.txt";
    let greeting_file_result = File::open(file_name);

    let greeting_file_name = greeting_file_result.expect("The specified file {file_name} doesn't exists");
}
```

### Error Propagation (Returning error to the caller function):
When a function’s implementation calls something that might fail, instead of handling the error within the function itself, you can return the error to the calling code so that it can decide what to do. This is known as propagating the error and gives more control to the calling code, 

