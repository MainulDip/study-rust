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