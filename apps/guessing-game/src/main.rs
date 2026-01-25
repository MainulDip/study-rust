#![allow(unused)]

use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let mut counter: u128 = 0;
    let time_old = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

    loop {
        counter += 1;
        // println!("counter = {counter}"); // no printing will make is 100x fast
        if counter == 10_000_000 {
            counter *= 2;
            break;
        }
    }

    println!("counter value is now {counter}");
    let time_now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    println!("Elapsed time = {} milliseconds", time_now - time_old);
}
