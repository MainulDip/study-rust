use std::println;

pub fn init_drop_trait() {
    println!("\n\n------------Intro to the drop trait with smart pointer-----------------------\n\n");
    main_drop();
}

fn main_drop() {
    println!("Calling drop function");
    let c1 = CustomSmartPointer {
        data: String::from("First Stuff")
    };

    let c2 = CustomSmartPointer {
        data: String::from("Second Stuff")
    };

    println!("CustomSmartPointers are created!");
} // after this ending braces, all CustomSmartPointers will be dropped and the implemented `drop` function will be called do other things
// note: the dropping will start from bottom to top, so the c2 will be dropped first, following the c1 will be dropped

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {}", self.data);
    }
}