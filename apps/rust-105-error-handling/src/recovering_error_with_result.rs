use std::{
    fs::File,
    io::{Error, ErrorKind},
};

pub fn recover_error_one() {
    println!("Bismillah");
    get_file_and_report();
    get_file_and_report_with_generics();
}

// Error (`Result<T, E>`) handling with nested match 
fn get_file_and_report() {
    let greeting_file_result = File::open("hello.txt");

    // handling Result's cases for accessing the hello.txt file, whether is exists or not
    let greeting_file = match greeting_file_result {
        Ok(File) => File,
        // Err(Error) => panic!("File not found: {Error_Msg}"), // instead of panicking here, let's create the file if not exists already
        Err(Error) => match Error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(File) => File,
                Err(Create_Failed) => panic!("File cannot be created: {Create_Failed:?}"),
            },
            _ => panic!("File not found: {Error:?}"),
        },
    };

    println!("{greeting_file:?}");
}


// Result<T, E> handling with closure function defined inside of the Result
fn get_file_and_report_with_generics() {
    let file_name = "Hello2.txt";
    let greeting_file_result = File::open(file_name);

    // handle Result's case with closure (unwrap_or_else) instead of multiple nested match statement 
    let greeting_file = greeting_file_result.unwrap_or_else( |error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(file_name).unwrap_or_else(|err| {
                panic!("Problem creating the file {err:?}");
            })
        } else {
            panic!("Problem opening the file {error:?}");
        }
    });

    println!("{greeting_file:?}");
}


// Result<T, E> handling with unwrap method 
// If the Result value is the Ok variant, unwrap will return the value inside the Ok. If the Result is the Err variant, unwrap will call the panic! macro for us. 
// unwrap cannot have custom panic message
fn get_file_and_report_with_unwrap() {
    let file_name = "Hello3.txt";
    let greeting_file_result = File::open(file_name);

    let greeting_file = greeting_file_result.unwrap();
}

// Result<T, E> handling with `expect`
// In production-quality code, most Rustaceans choose expect rather than unwrap. As unwrap doesn't have custom message
fn get_file_and_report_with_expect() {
    let file_name = "Hello4.txt";
    let greeting_file_result = File::open(file_name);

    let greeting_file_name = greeting_file_result.expect("The specified file {file_name} doesn't exists");
}