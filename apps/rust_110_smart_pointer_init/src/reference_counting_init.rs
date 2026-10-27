use std::rc::Rc;

pub fn reference_counting_init() {
    println!("\n\n------Introduction to reference counting using Rc<T>-------\n\n");
    main_reference_counting();
}

fn main_reference_counting() {
    println!("Reference counting main function call");

    // we cannot move value (`a` here) twice, the code below
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a)); // a cannot be moved again here

    // use Rc<T> instead to use any variable multiple times
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a)); // because we used Rc (reference counting), we can use the `a` as many time as we want, while also tracking the ownership
    // when nothing will be pointing to the `a`, it will be removed
    // Rc::clone is not a deep clone, so no performance loss like using a.clone only
}

// inside Cons, we're defining a recursive type
enum List {
    // Cons(i32, Box<List>),
    Cons(i32, Rc<List>),
    Nil,
}

use crate::reference_counting_init::List::{Cons, Nil};