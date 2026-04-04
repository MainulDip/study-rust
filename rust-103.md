### Structs:
A struct, or structure, is a custom data type that lets you package together and name multiple related values that make up a meaningful group.

It's like an data class for other oop languages (rust doesn't have class syntax)

Structs and enums are the building blocks for creating new types

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
}
```

### Struct 101:
- same ownership/borrow rules
- spread syntax
- variable mapping (Field Init Shorthand).......

* Use struct to add meaning by labeling the data (instead of tuple)

```rust
pub fn instantiate_struct_101() {
    let user_1 = User {
        _active: true,
        username: String::from("UserOne"),
        email: String::from("userone@email.com"),
        _sign_in_count: 1
    };

    println!("user_1 name: {} and email is: {}", user_1.username, user_1.email);

    // struct ownership/borrow follow the same principe
    // if the instantiated property is heap stored, it will be moved, so after using anything like that to build a new struct instance
    // the old instance cannot be used afterward if a heap stored value is mapped/used into new one (in simple terms)

    let user_2: User = User { _active: true, username: user_1.username, email: user_1.email, _sign_in_count: 1 };
    // now, the user_1 is no longer available
    // println!("user_1 name: {} and email is: {}", user_1.username, user_1.email); // error

    // we can also use the spread syntax like (json) to build a new struct from older struct
    // modified property should come before the spared call
    let user_3: User = User { username: String::from("UserThreeName"), email: String::from("userthree@email.com"), ..user_2  };
    println!("user_3 name: {} and email is: {}", user_3.username, user_3.email);
    // user_2 is still valid, because, we use `_active` and `_sign_in_count` from the user_2, which are stack stored (aka, has copy trait implemented)
    println!("user_2 name: {} and email is: {}", user_2.username, user_2.email);

    // build user using builder function
    let user_4 = user_builder(String::from("UserFourName"), String::from("userforu@email.com"));
    println!("user_4 name: {} and email is: {}", user_4.username, user_4.email);

    // if declared mutable, all properties get the mutability, no way to make partial mutable
    let mut user_5: User = User { _active: true, username: String::from("UserFiveName"), email: String::from("userfive@email.com"), _sign_in_count: 5 };
    println!("user_5 name: {} and email is: {}", user_5.username, user_5.email);
    user_5.email = String::from("newuserfive@email.com");
    println!("user_5's modified email is: {}", user_5.email);

    
    // building another user with the build_2 sending an already instantiate user
    let user_6 = user_builder_2(String::from("UserSixName"), String::from("usersix@email.com"), user_4);
    println!("user_6 name: {} and email is: {}", user_6.username, user_6.email); 

}

struct User {
    _active: bool,
    username: String,
    email: String,
    _sign_in_count: u64
}

// when a struct property and a variable have the same name, we can directly map the variable by only key (without the `key:val` syntax)
fn user_builder(username: String, email: String) -> User {
    User { _active: true, username, email, _sign_in_count: 1 }
}

fn user_builder_2(username: String, email: String, oldUser: User) -> User {
    User { username, email, ..oldUser }
}
```

### tuple struct:
These are structs that look like a tuple. Tuple structs don’t have names associated with their fields; rather, they just have the types of the fields.

* tuple structs are created and instantiate using `parenthesis` (not curly braces)

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

pub fn tuple_struct_call() {
    println!("\n--------tuple struct---------\n");

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    // destructuring tuple struct: requires to name the tuple struct's type
    let Point(x, _y, _z) = _origin;
    
    println!("Color props are {}, {}, {} and point.x is {}", _black.0, _black.1, _black.2, x );
}
```

### Unite-Like struct (to implement trait later without storing data in type itself):
Unite-like structs don't have any fields and behave similar to a `()` unit type.

* Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself.


* no parenthesis or curly braces to define and instantiate a unite-like struct


```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

### Struct and borrowed property (`&str`) and lifetime specifier:
Without using `lifetimes` struct cannot hold reference/borrowed types. By using owned type, each instance of the struct own all of its data and all those will be valid until the instance is valid (until moved or dropped).

* adding lifetimes [TODO]

### Derived Traits:
Using Derived Traits, we can print struct's properties for debugging purpose.

To do so, struct needs to explicitly opt in the functionality by adding outer attribute `#[derive(Debug)]` just before the struct's definition. Then we can use `{:?}`, `{struct:?}`, `{struct:#?}`, `{:#?}` and `dbg!(val_struct_or_borrowed_struct)`

* dbg! macro: it returns ownership of the expression, so for calculation, its same with or without. But we can also send reference inside of the `dbg!` macro to not get the ownership.

```rust
// adding the attribute to derive the Debug trait
// to print out a struct for debugging, we need to explicitly opt in to make that functionality available
#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32
}

pub fn struct_debug() {
    println!("\n---------Struct Debug Printing--------\n");
    let rect = Rectangle{height: 100, width: 100};
    println!("rect is {rect:?}");
    println!("rect is {rect:#?}"); // prettier than the `:?` version


    println!("rect printing with :?, so the rect is = {:?}", rect);
    println!("rect printing with :?, so the rect is = {:#?}", rect);


    let scale = 2;
    let rect2 = Rectangle {
        height: dbg!(30 * scale), // dbg! returns ownership of the expression’s value, the width field will get the same value as if we didn’t have the dbg!
        width: 30
    };
    println!("\n---------dbg!(&v)---------\n");
    dbg!(&rect2); // here, we are sending a reference, as we don't want to move the ownership
}  
```
