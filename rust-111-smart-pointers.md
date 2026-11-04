### Smart Pointer:
A pointer is a general concept for a variable that contains an address in memory. For rust, these are indicated by `&` and borrow the value they point to.

Smart pointers, on the other hand, are data structures that act like a pointer but also have additional metadata and capabilities. Rust has a variety of smart pointers defined in the standard library that provide functionality beyond the capabilities of pointer/borrowing.

With rust, references/pointers borrow the data, but in many cases smart pointers own the data they point to.


Smart pointers are usually implemented using structs and implementing the `Deref` and `Drop` traits.


`Deref` Trait: to customize the behavior of the dereference operator (*), enabling smart pointers and wrapper types to be treated like regular references. IE, writing `*x` on a custom type that implements deref trail, will run `*(x.deref())` automatically.


`Drop` trait: to customize the code that’s run when an instance of the smart pointer goes out of scope. 

Common Smart pointers in rust

- `Box<T>`, for allocating values on the heap
- `Rc<T>`, a reference counting type that enables multiple ownership
- `Ref<T>` and `RefMut<T>`, accessed through `RefCell<T>`, a type that enforces the borrowing rules at runtime instead of compile time


### `Box<T>` Smart pointer:
It's the most straightforward smart pointer. It allows to store data on the heap (instead of stack) and the pointer in the stack (memory address).

`Box<T>` is useful when
- a type size is unknown at compile time, and the context is requiring an exact size
- When you have a large amount of data, and you want to transfer ownership but ensure that the data won’t be copied when you do so
- When you want to own a value, and you care only that it’s a type that implements a particular trait rather than being of a specific type

* Boxes provide only the indirection and heap allocation; they don’t have any other special capabilities. Using Box with recursive type provide rust compiler the exact required data size prediction, as with Box<T>, it's only the memory pointer storage (same for everything) and the actual data will be stored on the heap



```rust
// const list (recursive pattern), ie, (1, (2, (3, Nil)))
enum List {
    // Cons(i32, List) // because we included another List type inside of the List type, it's a recursive type, without know size, rust will flag this as an error
    Cons(i32, Box<List>), // with the Box<T>, we've created a smart pointer, Now rust compiler knows, it only have to store the pointer address (fixed size for everything) to the stack memory and the actual data will be stored in the heap
    Nil,
}

use crate::box_smart_pointer::List::{Cons, Nil}; // without this shortcut creation, we'll have to call each of the Cons and Nil by List::Cons and list::Nil

// Calling the Cons list with different recursion levels
fn call_cons_list() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    let list3 = Cons(1, Box::new(Cons(2, Box::new(Nil))));
    let list2 = Cons(1, Box::new(Nil));
    let List4 = Nil;
    let list5 = List::Nil; // without using the short-cut created by `use` statement
}
```

* Dereference operator `*` (to convert pointer to actual value)

```rust
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
    // assert_eq(5, y); // will not work, as y is pointer here and `5` is a integer
}


// Same applies for Box<T>, after creating a Box type, it's a reference rather than an actual value
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

### `Deref` trait implementation with custom struct:
Any custom struct will not function when prefixed with `*` deref operator without implementing the `Deref` trait.

```rust
use std::ops::Deref; // importing the deref trait

pub fn custom_struct_deref_trait_implementation() {
    println!("\n\n----------Custom Struct With Deref Trait Implementation----------------\n\n");

    let x = 7;
    let y = MyBox::new(x);
    assert_eq!(7, *y);
    println!("*y = {}", *y); // *y = 7 // rust knows what to return when * is prefiex
    // println!("y = {}", y); // error, as rust doesn't know how to print y, will spit out error: ``MyBox<{integer}>` doesn't implement `std::fmt::Display``
    println!("x = {}", x); // x = 7 // still works as the x was stack allocated

    // lets try with string
    let x_string= String::from("hello");
    let y_string = MyBox::new(&x_string);
    assert_eq!("hello", *y_string); // without the the deref trait implementation, *y_string will not work
    println!("*y_string = {}", *y_string); // *y_string = hello
    println!("x_string = {}", x_string); // x_string = hello // still works as the string was borrowed, not moved
}

// Here, MyBox is a tuple struct, tuple struct cannot have named fields (keys), supports only value/s, no key/s.
struct MyBox<T>(T);

impl <T> MyBox<T> {
    // just like a constructor
    fn new(x: T) -> Self {
        MyBox(x)
    }
}

// implementing the deref trait so that MyBox can use `*` to dereference the value
impl<T> Deref for MyBox<T> {
    type Target = T;

    // notice, we're returning a reference here, as rust compiler will call the `*(y.deref())` behind the scene when we call `*y`
    // also returning a reference is required here, as it will not move the ownership
    fn deref(&self) -> &Self::Target {
        &self.0 // because, MyBox is a tuple struct, self.0 can be used to access the first value
        // note: rust compiler can evaluate the tuple struct's member, using self.1 will not be possible in this case. Compiler will flag error immediately.
    }
}
```

* Without the Deref trait, the compiler can only dereference `&` references. The deref method gives the compiler the ability to take a value of any type that implements Deref and call the deref method to get a reference that it knows how to dereference.

```rust
// behind the scene, while calling the *y, rust will run the below code automatically
*(y.deref())
// Note that the * operator is replaced with a call to the deref method and then a call to the * operator. 
// Rust compiler will do this automatically as it knows deref method returns a reference `&`, hence the `*()`, which ends up calling `*(y.deref())`
```

### Deref coercion (automatically implies on arguments to functions and methods):
In Rust, coercion is an implicit, automatic type conversion.

With the deref coercion (supplied by std library), the compiler automatically converts a reference to one type into a reference to another type if the source type implements the Deref trait.

For example, deref coercion can convert &String to &str, because String implements the Deref trait such that it returns &str.

Because of coercion, we can inject a String into our tuple struct `MyBox<T> (T)` and return a slice `&str` using the `*` dereference.

```rust
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m); // because MyBox implements Deref trait, we can do this easily
    // without deref coercion, we would have to do this
    hello(&(*m)[..]);
    // where, The (*m) dereferences the MyBox<String> into a String. Then, the & and [..] take a string slice of the String that is equal to the whole string to match the signature of hello
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}
```

* When the Deref trait is defined for the types involved, Rust will analyze the types and use Deref::deref as many times as necessary to get a reference to match the parameter’s type, all during compile time


### Deref coercion with mutable references:
Similar to Deref for immutable references, the `DerefMut` trait si used to override the * operator on mutable references.

Rust does deref coercion when it finds types and trait implementations in three cases:

1. From &T to &U when T: Deref<Target=U>
2. From &mut T to &mut U when T: DerefMut<Target=U>
3. From &mut T to &U when T: Deref<Target=U>

The first two cases are the same except that the second implements mutability. 

- The first case states that if you have a &T, and T implements Deref to some type U, you can get a &U transparently. 

- The second case states that the same deref coercion happens for mutable references.

- The third case states that if you have a &mut T, and T implements Deref to some type U, you can get a &U transparently.


Note: Rust will never coerce a mutable reference from an immutable reference. As this will brake the borrowing rules, `if you have a mutable reference, that mutable reference must be the only reference to that data (otherwise, the program wouldn’t compile).`


### Running Code on Cleanup with the `Drop` Trait:
The `Drop` trait is always used when implementing a smart pointer with custom type. It defines what happens when a value is went out of scope and dropped by compiler. Rust will call the drop function automatically. To manually drop an object, use `std::mem::drop` function with the object in the argument.

The `Drop` trait requires the `drop` method to be implemented.

```rust
fn main_drop() {
    println!("Calling drop function");
    let c1 = CustomSmartPointer {
        data: String::from("First Stuff")
    };

    let c2 = CustomSmartPointer {
        data: String::from("Second Stuff")
    };

    println!("CustomSmartPointers are created!");
} // after this ending braces, all CustomSmartPointers will be dropped and the implemented `drop` function will be called
// note: the dropping will start from bottom to top, so the c2 will be dropped first, following the c1 will be dropped

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {}", self.data);
    }
}
```

Note: Rust doesn’t let us call drop explicitly, because Rust would still automatically call drop on the value at the end of main. This would cause a double free error because Rust would be trying to clean up the same value twice.

* Manually dropping an `Instance` using the `(std::mem:drop)` as `drop` function. Used to cleanup values early, ie, managing locks by force deallocating a instance/value before the scope ends, so other code on the same scope can acquire the lock. 

```rust
fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created");
    c.drop();
    println!("CustomSmartPointer dropped before the end of main");
}
```


* The `Drop` trait and the `drop` function (`std::mem:drop`) are included in the prelude, so we don't need to bring that into scope.

### Rc<T> Reference Counting Smart Pointer (Single Threaded):
To enable multiple owners of a single value, the `Rc<T>` smart pointer is used. 

Like in a graph data structure, multiple edges might point to the same node, and that node conceptually owned by all of the edges that point to it. A node node shouldn't be cleaned up unless it doesn't have any edges pointing to it and so has no owners.

The Rc<T> type keeps track of the number of references to a value to determine whether or not the value is still in use. If there are zero references to a value, the value can be cleaned up without any references becoming invalid.

```rust
use std::rc::Rc; // we need to bring Rc<T> into scope, as it's not in the prelude

// inside Cons, we're defining a recursive type
// #[derive(Debug)] // if we want to print the emum on the fly
enum List {
    // Cons(i32, Box<List>), // Box will not work if we want to reference a variable multiple time without deep cloning. Also Rc::clone is not a deep clone, used when contracting this enum
    Cons(i32, Rc<List>),
    Nil,
}

use crate::reference_counting_init::List::{Cons, Nil}; // creating shortcut for easy access

fn main_reference_counting() {
    
    // we cannot move value (`a` here) twice, in the code below, we moved `a` twice (first in `b` and then in `c`), which is not allowed 
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a)); // compile error: a cannot be moved again (we already moved once on the `b` definition)

    // use Rc<T> instead to use any variable multiple times
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a)); // because we used Rc (reference counting), we can use the `a` as many time as we want, while also tracking the ownership
    // when nothing will be pointing to the `a`, it will be removed
    // Rc::clone is not a deep clone, so no performance loss like using a.clone only
    // The call to Rc::clone only increments the reference count, which doesn’t take much time
}
```


* Inner workings of Rc<T>, by printing stored reference number using `Rc::strong_count(&Rc<T>);

```rust
// --snip--

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

// count after creating a = 1
// count after creating b = 2
// count after creating c = 3
// count after c goes out of scope = 2

// because, we have created inner scope in the main function, when the scope ends, the variable dropped and Rc::strong_count get reduced
// after the main function ends, everything will be cleared up
```


* printing Rc<T>


```rust
// implement printing with loop function
fn print_with_loop(list: &List) {
    let mut current = list;
    while let Cons(val, next) = current {
        println!("Value = {val}");
        current = next;
    }
}

// implementing printing with recursive function
fn print_with_recursion(list: &List) {
    match list {
        Cons(val, next) => {
            println!("Value = {val}");
            // print_with_recursion(&next); // works, but `&` before `next` is redundant, as it's already been borrowed
            print_with_recursion(next); // this will also work, as next is already borrowed
            print_with_recursion(next.as_ref()); // this will also work (though redundant), as Rc<T> already implements AsRef trait
        },
        Nil => (),
    }
}

// experiment: manual deconstruction
fn manual_decons() {
    let v = Some(String::from("Hello"));
    let inner_borrow = match v {
        // note: using `ref` keyword here to bind a variable by reference rather than moving the value
        Some(ref s) => {
            println!("{s}");
            Some(s)
        },
        None => None,
    };
}
```

* Using Rc<T>, we can only borrow immutably. To mutate use interior mutability pattern with `RefCell<T>` mixing with Rc<T>

### `as_ref()` and it's alternatives:
The .as_ref() method comes from the AsRef trait. It is an explicit, manual conversion designed to let a type cheaply represent itself as a reference to a different type.

It is highly useful in generic programming where you want to accept anything that can look like a `&str`, `&[u8]`, or `&Path`.

```rust
fn print_generic<T: AsRef<str>>(s: T) {
    println!("{}", s.as_ref());
}

let variable: String = String::from("hello");
print_generic(variable); // Works because String implements AsRef<str>
```

* The problem: Sometimes borrowing an `Option<T>` with `&` into a variable, borrows the whole option as `&Option<T>`. To achieve `Option<&T>`, `.as_ref()` comes handy

```rust
let v: Option<String> = Some(String::from("hello"));

// using standard borrowing with `&`
let outer_borrow: &Option<String> = &v;

// using `.as_ref()` to borrow internally
let inner_borrow: Option<&String> = v.as_ref();
// though `Option` doesn't implement `AsRef` trait, `as_ref()` is chainable because
// `Option` has its own method definition for `as_ref`, which returns `Option<&T>
```

* Alternatives: manual deconstruction by `match`

```rust
fn manual_decons() {
    let v = Some(String::from("Hello"));
    let inner_borrow = match v {
        // note: using `ref` keyword here to bind a variable by reference rather than moving the value
        Some(ref s) => {
            println!("{s}");
            Some(s)
        },
        None => None,
    };
}
```

* Alternatives: Conditional Assignment by `if let`

```rust
fn conditional_assignment() {
    let v = Some(String::from("hello"));
    // note: using the same `ref` keyword for reference binding
    let inner_borrow = if let Some(ref s) = v {
        Some(s)  
    } else { // note: for statement or assignment, `else` block is mandator for `if let` statement
        None
    };
}
```

* Alternatives: Borrowing the whole option first then mapping using `map(<closure>)`

```rust
let v: Option<String> = Some(String::from("hello"));
// `&v` returns &Option<String>, and .map handles the conversion.
let inner_borrow: Option<&String> = (&v).map(|s| s);
// Note: if you want `Option<&str>`, use `.as_deref()`
```


### Interior Mutability Pattern (`unsafe`) and `RefCell<T>`:
Its a design pattern that allow to mutate the data even when there are immutable reference to that data, which on normal is disallowed by the borrow checker. It use `unsafe` inside a data structure to bend Rust usual rules (borrow checker).

