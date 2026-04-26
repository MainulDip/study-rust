///*
/// We define a module with the mod keyword followed by the name of the module (in this case, front_of_house). 
/// The body of the module then goes inside curly brackets. Inside modules, we can place other modules, as in this case with the modules hosting and serving. 
/// Modules can also hold definitions for other items, such as structs, enums, constants, traits, and as in Listing 7-1, functions.
///  */

mod front_of_house {
   pub mod hosting {
        pub fn add_to_waitlist() { println!("Hello from add_to_waitlist") }
        fn seat_at_table() { println!("Hello from seat_at_table") }
    }

    mod service {
        fn take_order() { println!("Hello from take_order") }
        fn server_order() { println!("Hello from server_order") }
        fn take_payment() { println!("Hello from take_payment") }
    }
}


pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();
}

// super with relative path: super can be used to call parent defined (outer scoped function) from inside a scoped module 
fn some_other_task() {
    println!("Do something now");
}

mod back_of_house {
    fn fix_incorrect_order() {
        super::some_other_task();
        cook_order();
    }

    fn cook_order() { println!("Cooking the food") }
}