use std::{print, println, rc::Rc};

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

    // print the values
    // println!("a = {a:?}");
    // println!("b = {b:?}");
    // println!("c = {c:?}");

    
    // printing using looping function
    print_with_loop(&c);
    print_with_loop(&b);
    print_with_loop(&a);


    // printing with recursion
    println!("\n\nPrinting with recursion -------------------\n");
    print_with_recursion(&c);

}

// inside Cons, we're defining a recursive type
// #[derive(Debug)]
enum List {
    // Cons(i32, Box<List>),
    Cons(i32, Rc<List>),
    Nil,
}

use crate::reference_counting_init::List::{Cons, Nil};

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