#![allow(unused)] // to get rid of Unsed warning

use std::io; // std::io:* to bring all packages
use rand::Rng; // packages/libraries are called crates in rust, search rust libraries and version in crates.io
use std::io::{Write, BufReader, BufRead, ErrorKind}; // nested path syntax to multiple import
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    println!("Bismillah");
    println!("what is your name?");

    // by default variables are immutable, specify `mut` to make it mutable
    let mut name: String = String::new();
    let greeting: &str = "Nice to meet you";
    io::stdin().read_line(&mut name)
    .expect("Didn't Receive Input");
}
