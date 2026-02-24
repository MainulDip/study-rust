#![allow(unused)]
mod ownership_and_function;
mod return_value_and_variable_scope;
mod reference_and_borrowing;

// use std::io;
use ownership_and_function::ownership_and_function;
use return_value_and_variable_scope::return_value_and_variable_scope;
use reference_and_borrowing::reference_and_borrowing;

fn main() {
    ownership_and_function();
    return_value_and_variable_scope();
    reference_and_borrowing();
}
