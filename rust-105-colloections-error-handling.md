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
let string_slice_not_valid = &s[0..1];
println!("{string_slice_not_valid}"); // will panic as the first byte is not a valid character, it needs 3 byte to form character "ক"
```


### Hash Map (Key value pair, like map or dictionary or associative array):