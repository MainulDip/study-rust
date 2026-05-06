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

### Struct and Methods:
Methods are defined in the context of a struct (or an enum or a trait object) and their first parameter is always `self` (`&self` or `&mut self`), which represents the instance of the struct the method is being called on.

- the first parameter of the method (or if only param) will always be `self`
- methods are defined inside of a `impl` (implementation) block
- in method parameter, `&self` is shorthand for `self: &Self`.
- within `impl` block, Self is an alias for the type that impl block is for
- that `self` can be `&self` reference, or `self` ownership type. Also can be `&mut self`. As Methods can take ownership of self, borrow self immutably, or borrow self mutably, just as they can any other parameter.
- method/s can have same name as prop/s. Rust compiler will detect the difference by detecting the caller parenthesis.
- we can have multiple `impl` blocks

* method with ownership type `self` as the first parameter is rare; this technique is usually used when the method transforms self into something else and you want to prevent the caller from using the original instance after the transformation.

```rust
pub fn structs_method_explore() {
    println!("\n------------Struct's Method Exploring----------------\n");
    let rect = Rectangle { height: 100, width: 100};
    println!("Rectangles area is = {}, using method", rect.area());
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// we can have multiple `impl` blocks
impl Rectangle {
    fn is_width_valid(&self) -> bool {
        if self.width > 0 { true } else { false } // rust doesn't have `condition ? exp : exp` like ternary syntax
    }

    // we can have same name as prop
    fn width(&self) -> u32 {
        // get the width of the Rectangle
        if self.width > 0 { self.area() / self.height } else { 0 }      
    }
}
```


### Automatic referencing and de-referencing:
In C and C++, two different operators are used for calling methods: You use . if you’re calling a method on the object directly and -> if you’re calling the method on a pointer to the object and need to dereference the pointer first. In other words, if object is a pointer, object->something() is similar to (*object).something().

Rust doesn’t have an equivalent to the -> operator; instead, Rust has a feature called automatic referencing and dereferencing. Calling methods is one of the few places in Rust with this behavior.

Here’s how it works: When you call a method with object.something(), Rust automatically adds in &, &mut, or * so that object matches the signature of the method. In other words, the following are the same:

```rust
// both are same
p1.distance(&p2);
(&p1).distance(&p2);
```

### Associated/non-method function (functions without self as 1st parameter):
All functions defined within an impl block are called associated functions because they’re associated with the type named after the impl. We can define associated functions that don’t have self as their first parameter (and thus are not methods) because they don’t need an instance of the type to work with. IE, String::from function that’s defined on the String type.

* Associated functions that aren’t methods are often used for constructors that will return a new instance of the struct. These are often called new, but new isn’t a special name and isn’t built into the language.

```rust
impl Rectangle {
    // To call this associated function, we use the :: syntax with the struct name
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let sq = Rectangle::square(3)
}
```


### Enumerations (`enum`), Pattern matching (`match`), `Option` and `if let`:
Enums allow to define a type by enumerating its possible variants, ie, Option enum.

Where structs give you a way of grouping together related fields and data, like a Rectangle with its width and height, enums give you a way of saying a value is one of a possible set of values. 

Say we need to work with IP addresses. Currently, two major standards are used for IP addresses: version four and version six. Because these are the only possibilities for an IP address that our program will come across, we can enumerate all possible variants, which is where enumeration gets its name.

Any IP address can be either a version four or a version six address, but not both at the same time. That property of IP addresses makes the enum data structure appropriate because an enum value can only be one of its variants. Both version four and version six addresses are still fundamentally IP addresses, so they should be treated as the same type when the code is handling situations that apply to any kind of IP address.

```rust
enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind) {}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    if is_ipv4 { route(four) } else { route(six) }
}
```

### enum with struct:

```rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1");
}

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("[::1]"),
}
```

### enum with associated value:
Enum instance can also hold associated values of any type. But to read those values latter, we need to our own way defined using match statement.


```rust
enum IpAddr {
    V4(String),
    V6(String),
}

// we need custom mechanism to print or read enum's associated values, as enum does not come with a display trait implemented for its associated values
impl IpAddr {
    fn readValue(&self) -> String {
        match self {
            IpAddr::V4(v) => v.to_string(),
            IpAddr::V6(v) => v.to_string(),
        }
    }
}

fn main {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("[::1]"));

    println!("Home ipv4 localhost is {}", home.readValue());
    println!("Home ipv6 localhost is {}", loopback.readValue());
}
```

* note - `::1` is the ipv6 address for localhost (127.0.0.1 is ipv4). To navigate inside of a browser, use `http://[::1]` or with port `http://[::1]:8080`. The `[]` is crucial, as it stop the browser to think it as a port number.

### Enum and Struct:
Enum provides grouping mechanism with lesser code than writing multiple struct for same functionality. 

* with enum, we can write functions to match the enumerated types and compute, but with multiple different structs, it not possible to do so.

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}


// Struct equivalent of the `Message` enum

struct QuitMessage; // unit struct
struct MoveMessage { // struct with named field (regular struct)
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

```

### enum methods:
Enum's methods are defined the same way as struct's, using `impl` block.

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

impl IpAddr {
    fn readValue(&self) -> String {
        match self {
            IpAddr::V4(v) => v.to_string(),
            IpAddr::V6(v) => v.to_string(),
        }
    }
}


fn main {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("[::1]"));

    println!("Home ipv4 localhost is {}", home.readValue());
    println!("Home ipv6 localhost is {}", loopback.readValue());
}
```


### Option enum (std library):
Rust does not have nulls, but it does have an enum `Option<T>` that can encode the concept of a value being present or absent.

The `Option<T>` enum is included in the prelude; you don’t need to bring it into scope explicitly. Its variants `Some` and `None` are also included in the prelude and can be used directly without the Option:: prefix. The Option<T> enum is still just a regular enum, and Some(T) and None are still variants of type `Option<T>`.......

```rust
enum Option<T> {
    None,
    Some(T),
}

fn main() {
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
}
```

### Option<T> operation with simple Value type:

```rust
pub fn option_handle_a() {
    println!("\n----------------Option<T> Handling------------------\n");
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    match y {
        Some(v) => println!("v + x = {}", v + x),
        None => println!("y is None"),
    }


    println!("option_unwrapping : {}", option_unwrapping());
    println!("option_if_let : {}", option_if_let());

    match option_with_map() {
        Some(val) => println!("match Some arm's value is {val}"),
        None => println!("Match None arm executed"),
    }
}

fn option_unwrapping() -> i8 {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y.unwrap_or(0);
    sum
}

fn option_if_let() -> i8 {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    if let Some(val) = y {
        let sum = x + val;
        sum
    } else {
        let sum = x + 0;
        sum
    }
}

fn option_with_map() -> Option<i8> {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = y.map(|val| x + val);
    sum
}
```

### `match` Control Flow:
It's the `switch` eqevalent for language like `swift`, `kotlin`.
Works best with enum. 

`match` statement is consist of arm/s. An arm has two parts: a pattern (for matching case) and some code (to execute), separated by `=>`. Multiline codes are placed using `{}`, when braces are used, `,` after each arm becomes optional.

```rust
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // others
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        } // multiline, as braces are used, comma is optional here
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}
```


### `Option<T>` and match:

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

### `other` as Catch-All Pattern and the `_` placeholder:
`match` needs to provide all possible cases. When we're not evaluating enum, we have to provide default/fallback case by using `other` or a underscore `_`.

* `other` when we need to use the injected value, and `_` when we don't need.

```rust
let dice_roll_first = 9;

match dice_roll_first {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}


let dice_roll_first = 7;

match dice_roll_second {
    7 => println!("7"),
    _ => println!("Something else"),
}

match dice_roll_second {
    7 => println!("7"),
    _ => (), // when we're not going to execute anything, just use () unit type
}
```

### Concise control flow using `If let` and `let...else`:
if-let is for handling matched value and ignoring the rest. Its kinda syntactic sugar as we need less boilerplate. 

```rust
// using `if let`
let config_max = Some(3u8);
if let Some(any_value) = config_max {
    println!("The maximum is {any_value}");
}

// same using match, which use more code
match config_max {
    Some(any_value) => println!("The maximum is {any_value}"),
    _ => (),
}
```

`if let` can also followed by and `else`, which will act similar to the match's `_` case (handling other/fallback case without caring about the matched value)

```rust
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {state:?}!");
} else {
    count += 1;
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // others
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
```


### `if let` when returning value and binding:
The common pattern is to perform some computation when a value is present and return a default value otherwise.

* `let...else` is better than this version, see below

```rust
mpl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // -- snip --
        }
    }
}

// using if let to match on the type of coin, introducing a state variable within the body of the condition
fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}

// can be re-written (more readable)
fn describe_state_quarter(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
```

### Happy path `let...else` for returning value and binding with variable:

The `let...else` syntax takes a pattern on the left side and an expression on the right, very similar to if let, but it does not have an if branch, only an else branch. If the pattern matches, it will bind the value from the pattern in the outer scope. If the pattern does not match, the program will flow into the else arm, which must return from the function.......

```rust
// this function will return either `Some(String)` or `None`
fn describe_state_quarter(coin: Coin) -> Option<String> {
    // If the provide coin is `Coin::Quarter(state)`, the state variable will be available for later use, otherwise, it will follow the else fallback, which is returning `None` here
    let Coin::Quarter(state) = coin else {
        return None;
    };

    // if the previous block binding (state variable) is success, this if-else block will be evaluated
    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
```