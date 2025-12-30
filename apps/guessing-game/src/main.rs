use std::io;
// importing a library is called `prelude` in rust. prelude = `an action or event serving as an introduction to something more important`
// std stands for `standard` library. `std::io` makes the input/output standard library available to the scope

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    // The equal sign (=) tells Rust we want to bind something to the variable
    let mut guess = String::new();

    /*
    * The `::` syntax in the `::new` line indicates that new is an associated function of the `String` type. 
    * An associated function is a function that’s implemented on a type, in this case String. 
    * You’ll find a new function on many types because it’s a common name for a function that makes a new value of some kind.
    * If we hadn’t imported the io module with use std::io; at the beginning of the program, we could still use the function by writing this function call as std::io::stdin. 
    * The stdin function returns an instance of std::io::Stdin, which is a type that represents a handle to the standard input for your terminal.
    */

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

    // The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data 
    // without needing to copy that data into memory multiple times. 
    // like variables, references are immutable, so we need to specify `&mut` to mutate

    /*
     * read_line function returns a Result type. Result is an enumeration, often called an enum, which is a type that can be in one of multiple possible states.
     * Result’s variants are Ok and Err. The Ok variant indicates the operation was successful, and it contains the successfully generated value. The Err variant means the operation failed, and it contains information about how or why the operation failed.
     * 
    */
}
