### Ownership and Borrow:

Docs : https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html and follow part 2 and 3 also

Ownership is the rust's most unique feature, it enables Rust to make memory safety guarantees without needing a garbage collector. Ownership model work together with `borrowing, slices and how rust lays data out in memory`.

Stack vs Heap: 

* All data stored on the stack must have a known, fixed size. Data with an unknown size at compile time or a size that might change must be stored on the heap instead.

Stack's data is organized, maintain the `LAFO` pattern, last-in-first-out. 

The heap is less organized. When you put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location. This process is called allocating on the heap and is sometimes abbreviated as just allocating (pushing values onto the stack is not considered allocating). Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer.


When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function’s local variables get pushed onto the stack. When the function is over, those values get popped off the stack.


Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so that you don’t run out of space are all problems that ownership addresses. Once you understand ownership, you won’t need to think about the stack and the heap very often. But knowing that the main purpose of ownership is to manage heap data can help explain why it works the way it does. 


Ownership Rules:

- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

```rust
fn main() {                      
    // s is not valid here, since it's not yet declared
    let s = "hello";   // s is valid from this point forward
    let t = "world";   // t is valid from here
}                      // this scope is now over, and s and t are no longer valid, t removed first, then the s had been removed. As of `stack`, last-in-first-out.
```

### String vs String literal:

* Learn difference between Rust core vs std library first.

Overall `String` type can refer to either core language's string slice `&str` borrowed type or the std library's `String` (owned) type.

Rust core library comes with only one string type, string slice `str` that is usually seen in its borrowed form, `&str`. This string slices are references to some UTF-8 encoded string data stored elsewhere. String literals, for example, are stored in the program’s binary and are therefore string slices.

The std library provide the `String` type (different from the core's string slice type), which is a growable, mutable, owned, UTF-8 encoded string type, stored in the heap.  


* example 

```rust
let string_from_literal_1: &str = "Hello world";
let string_from_literal_2: String = "Hello world for std library's String";
```

### Rust core vs standard-library:
The Rust core library is the minimal, platform-agnostic foundation of the language, while the `std` library is the full standard library that builds upon core and adds platform-dependent capabilities like I/O and networking.

The core is not aware of features like heap allocation, concurrency, or file I/O, as these depends on OS.


### Rust namespace: 
A namespace is a logical grouping of declared names. Namespaces allow the occurrence of a name in one namespace to not conflict with the same name in another namespace. In layman's terms, namespaces are used to organize code and prevent naming conflicts.

```rust
// ---------------------- math.rs file ------------------------------
// Functions are public so they can be accessed from outside the module
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}


// ------------------------- main.rs file --------------------------
mod math; // Declares the module from the file

use math::add; // Bring only the 'add' function into scope
// use math::* // Bring all public items into scope

fn main() {
    let sum = add(5, 3); // Can now call 'add' directly
    let difference = math::subtract(5, 3); // Still need to qualify 'subtract'
    println!("Sum: {}, Difference: {}", sum, difference);
}
```

- `mod` : Declares a new module, creating a new namespace.
- `pub` : Makes items (functions, structs, etc.) public, allowing them to be accessed from outside their current module.
- `use` : Imports items into the current scope, so you don't have to use their full path every time.
- `::` : The namespace separator, used to access items within a module (e.g., std::collections::HashMap). 

### Ownership (heap) | coping is actually moving the ownership:
A String of made of 3 parts (std String)
- a pointer to the stack memory, that holds the content of the String (data)
- a length (actual) of the String
- a capacity, the total amount of memory in bytes that the allocator has allocated for the String  

When a heap-stored data, ie, `String` type of the standard library, is copied into another variable, (now) the new variable own the data, not the old variable. Its the heap stored data (pointer, length, capacity), not the stack stored actual data. 

* double free error: when two variables are pointing to the same data (copied to variable-2 from variable-1), when this 2 variables go out-of-scope, they will both try to free the same memory. Rust solve this by only making the later variable valid (owner)

```rust
// this will not work
let s1 = String::from("hello");
let s2 = s1;

// this will not work, as after copied, only s2 is valid. s1 is not valid
println!("{s1}, world!");
```

* shallow vs deep copy: copying the pointer, length, and capacity without copying the data is shallow copy. And coping everything is a deep copy. But rust's behavior is more like a `move`, where the shallow data is transferred/moved to a new variable and the old variable become obsolete immediately.

* Rust will never automatically create “deep” copies of your data. Shallow copy will always invalidate the old variable. To do deep copy, use `clone` method.

For mutable variables, when a new value is assigned to a mutable variable, Rust will run the drop function and the old value will be freed up immediately. (guess, the allocator will write new data to the stack and modify the mutable variable with new pointer, length and capacity)


```rust
let s1 = String::from("hello");
let s2 = s1.clone(); // deep copy 

println!("s1 = {s1}, s2 = {s2}"); // valid 
```

### Stack only Data: Copy trait:
Types that have known size at compile time are stored entirely on the stack, as stack stored data is quick to make, coping data to a new variable perform an actual copy (not move like heap stored type). After coping, the old variable doesn't get invalid. Also there is no difference between shallow and deep copy in this case, all are same, also calling `clone` method will not do any special that the default copy.

```rust
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");
```

* copy trait: Usually all stack-stored data types already comes with `copy` trait implemented. So they can be copied rather than move. Custom type can also implement `copy` trait, unless the type or parts of it implement `drop` trait (heap stored move).


* copy trait implemented types

- All the integer types, such as u32.
- The Boolean type, bool, with values true and false.
- All the floating-point types, such as f64.
- The character type, char.
- Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

### Ownership and function:
Same ownership rules apply for function. If heap-stored type is passed as function's parameter, the ownership changes, and the previous variable will not be valid afterwards. Unless returned and captured by another variable, the results will get lost (as rust runtime will executed drop method for the previous variable)

