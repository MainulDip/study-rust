// #![allow(unused)]

use std::io;


fn main() {

    let mut input_x: &str = "";
    let mut input_y: String = String::new();

    io::stdin()
    .read_line(&mut input_y)
    .expect("something went wrong");

    input_x = &input_y;
    println!("input_x = {input_x}");

    println!("input_x = {input_x}");


    let num_array = [1, 2, 3, 4, 5, 6, 7];

    let mut s: &str = "Hello as borrowed string";
    println!("s = {s}");

    s = "1st mutation for s";
    println!("new string = {s}");

    let mut s2 = s;
    println!("1st s2 = {s2}");
    s2 = "1st mutation for s2";
    println!("2nd s2 = {s2}");

    s = "3rd mutation for s";
    println!("s now = {s}");
    println!("checking the s2 = {s2}");



    for element in num_array {
        println!("printing {element}");
    }

    // with both index and element
    for (index, element) in num_array.iter().enumerate() {
        println!("num_array index: {index} and element: {element}")
    }

    // for with reverse iteration
    println!("Printing reverse order");
    for element in num_array.iter().rev() {
        println!("printing {element}");
    }

    // `for` to iterate over range in reverse order
    for element in (1..=7).rev() {
        println!("printing {element}");
    }
}
