#![allow(unused)] // to get rid of Unused warning

use rand::Rng; // packages/libraries are called crates in rust, search rust libraries and version in crates.io
use std::cmp::Ordering;
use std::fs::File;
use std::io; // std::io:* to bring all packages
use std::io::{BufRead, BufReader, ErrorKind, Write}; // nested path syntax to multiple import

fn main() {
    // Emum
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    // defining functions for enum
    impl Day {
        fn is_weekend(&self) -> bool {
            return match self {
                Day::Saturday | Day::Sunday => true,
                _ => false,
            };
        }
    }

    // usages
    let today: Day = Day::Monday;
    match today {
        Day::Monday => println!("Everyone hates Monday"),
        Day::Tuesday => println!(""),
        Day::Wednesday => println!(""),
        Day::Thursday => println!(""),
        Day::Friday => println!(""),
        Day::Saturday => println!(""),
        Day::Sunday => println!(""),
    }

    println!("Is today a weekend = {}", today.is_weekend());
}
