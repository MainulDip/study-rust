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
    println!("**y_string = {}", **y_string);
    
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

    fn deref(&self) -> &Self::Target {
        &self.0 // because, MyBox is a tuple struct, self.0 can be used to access the first value
        // note: rust compiler can evaluate the tuple struct's member, using self.1 will not be possible in this case
    }
}

