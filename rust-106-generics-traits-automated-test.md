### Covered Topics and Next:
- Generics
- Traits
- Lifetimes
- Automated Tests
- Functional Programming | Iterators & Closures
- Smart Pointers
- Fearless Concurrency
- Asynchronous programming | Async/Await, Futures & Streams
- OOP
- Patterns and Matching
- Advance Features | Unsafe Rust, Advanced Traits,  Advanced Types
- Advanced Functions and Closures
- Macros | After the Fundamentals -> Little Book of Rust Macro | https://lukaswirth.dev/tlborm/introduction.html | Do it on you own pace
- Building a web server and make that multi threaded

### Generics Functions:
We use generics to create definitions for items like function signatures or structs, which we can then use with many different concrete data types.

```rust
/**
* here both largest_i32 and largest_char are non-generics, but function the same. They both find the largest from numbers or chars vector/list
* the largest_finding_generics function is the combination of both using generics with a single function definition
* note: because we're comparing values, we need bound T with the `std::cmp::PartialOrd` trait
*/

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// implementing generics for finding largest number or char
fn largest_findings_generics<T: std::cmp::PartialOrd> (list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
```

### Generic Struct (+ Enum):
```rust
pub fn generic_struct_introduction() /* -> Point<i32, f64> */ {
    println!("\n\n------------Generic Struct Playground-------------------\n\n");
    let point = Point{ width: 100, height: 100.4 };
    println!("point = {point:?}");
    println!("point width = {} and height = {}", point.width, point.height);
    /* point */
}


// create a Generic Point struct, with two different Generic types
#[derive(Debug)]
struct Point<T, U> {
    width: T,
    height: U
} 

// Note: For same file, private fields can be accessed (unless behind a module)
// But to access the struct from another file, the struct needs to be public (pub)
// Also, to access the field outside, it needs to be public as well
// Like, if the public facing function return a Point object, and we capture that from main function, it needs to modify all those private fields to public
/*
 * let mut sth = generic_struct_introduction(); // will not work unless we make the Point struct public
 * sth.width = 200.4; // won't work unless we make the width public
 */
```

* The Option<T> and Result<T, E> are generic Enum

### Generic method:
We need to declare generic type after impl `impl<T> Struct_name<T>`, so that, the compiler can identify the generic type (rather than a concrete type).

* a method within an impl that declares a generic type (`impl<T> Struct_name<T>`), that method will be defined on any instance of the type, no matter what concrete type ends up substituting for the generic type. 

```rust
struct Point<T> {x: T, y: T,}
impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point{x: 7, y: 7};
    println!("p.x = {}", p.x()) // p.x = 7
}
```

* But impl defined for concrete type is only available for that only (`impl Struct_name<i32>`), which we can use to define constrain for a generic struct type

```rust
// here the impl block block only applies to a struct with a particular concrete type (f32) for the generic type parameter T
// so when T is not a f32 floating point, the method `distance_from_origin` will not be available
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

* Different generic types for method (different that impl and struct's definition): Methods can have different generic types. which are bound to that method only.

```rust
#[derive(Debug)]
struct Container<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Container<X1, Y1> {
    // different generic type (than the impl and struct) are bound to the method only
    // it can be used to create constraints for that method, ie, here, we're restricting to return values from same container and also differentiating other_container signature from the self container signature

    fn mixup<X2, Y2>(self, other_container: Container<X2, Y2>) -> Container<X1, Y2> {
        Container { x: self.x, y: other_container.y }
    }
}

pub fn call_generic_struct_method() {
    println!("\n\n--------------Generic Struct Method------------------------\n\n");
    let container_1 = Container { x: 7, y: 7 };
    let container_2 = Container { x: "Hello", y: "World" };
    let final_container = container_1.mixup(container_2);
    println!("Mixed Up Container is {final_container:?}");
}
```

### Generic types and performance:
Rust generics don't create any performance drop. At compile time, all generic types are converted to concrete definition, which is called `Monomorphization`

`Monomorphization` = `Mono` + `Morphing` = Morphing to mono/singular

* Example: the `Option<t>` can hold different concrete types. At compile time, all those are converted 

```rust
let integer = Some(5);
let float = Some(5.0);

// The generic Option<T> is replaced with the specific definitions created by the compiler.

enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

### Traits (interface with some differences):
A trait defines the functionality a particular type has and can share with other types. We can use traits to define shared behavior in an abstract way. We can use trait bounds to specify that a generic type can be any type that has certain behavior.

* A trait can have multiple methods in its body: The method signatures are listed one per line, and each line ends in a semicolon.

* Traits can have both abstract and non-abstract (default) methods

```rust
// Defining the Summary trait with an abstract method
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticles {
    pub headline: String,
    pub location: String,
    pub author: String, 
    pub content: String,    
}
```