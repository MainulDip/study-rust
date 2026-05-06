### Common (heap-stored) Collection Type (other than built-in stack-stored array and tuple types):

- `vector: Vec<T>`: stores variable number of values next to each other
- `String`: collection of characters (from std library)
- `hash map`: allow to store value with key (key-value pair), `map` for other programming language.



### Vectors `Vec<T>`:
It's a heap stored collection that can store more than one value of the same type.

- Creating an empty vector `Vec::new()` and vector with initial values use `vec![1, 2, 3]` macro.

```rust
let empty_vector: Vec<i32> = Vec::new();

let mut v = vec![1, 2, 3, 4, 5, 6];

```