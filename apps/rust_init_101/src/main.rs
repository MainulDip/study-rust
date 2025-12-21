#![allow(unused)] // to get rid of Unused warning

use rand::Rng; // packages/libraries are called crates in rust, search rust libraries and version in crates.io
use std::cmp::Ordering;
use std::fs::File;
use std::io; // std::io:* to bring all packages
use std::io::{BufRead, BufReader, ErrorKind, Write}; // nested path syntax to multiple import
use std::ops::Add; // operators Add to implement Generics computation

fn get_sum_generics<T: Add<Output = T>> (x: T, y: T) -> T {
    // to do any further computation in the Generics Type, we need to set bounds using Traits (Interfaces)
    return x + y;
}
fn main() {
    // Generics
    
    println!("5 + 4 = {}", get_sum_generics(4, 7));
    println!("5 + 4 = {}", get_sum_generics(4.12, 7.12));
}
