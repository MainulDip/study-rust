#![allow(unused)] // to get rid of Unused warning

use rand::Rng; // packages/libraries are called crates in rust, search rust libraries and version in crates.io
use std::cmp::Ordering;
use std::fs::File;
use std::io; // std::io:* to bring all packages
use std::io::{BufRead, BufReader, ErrorKind, Write}; // nested path syntax to multiple import

fn main() {
    // Vector: Vectors are like array. Mutable vectors can grow. It store value of same type
    let vec1: Vec<i32> = Vec::new();
    let mut vec2:Vec<i32> = vec![1,2,3,4];
    vec2.push(5); 
    println!("1st : {}", vec2[0]);

    let second: &i32 = &vec2[1];
    match vec2.get(1) {
        Some(second) => println!("(Matching)  2nd : {}", second),
        None => println!("Not a match!"),
    };

    // note : if Some and None arms are provide inside of the match block, the match will expect `Option<T>`
    match vec2[1] {
        // Some(second) => println!("(Matching)  2nd : {}", second),
        // None => println!("Not a match!"),
        _ => println!("Just a default output") // default arm must be provide for non exhaustive matching
    };

    for i in &mut vec2 {
        // `&mut vec2` will make the vec2 borrow-able, so we can modify that inside of the for loop
        *i *= 2; // multiplying the element (pointer) with 2 
    }

    // lets check if the vec2 is modified

    // checking with index, using vec2.inter().enumerate()
    for (index, element) in vec2.iter().enumerate() {
        println!("vec2 index : {} and vec2 element : {}", index, element);
    }
}
