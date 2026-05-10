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