use std::{fs::File, io::{self, Read}};

pub fn propagating_error_fn() {
    println!("Propagating Errors To Its Caller Function");
    match read_username_from_file("Hello7") {
        Ok(content) => println!("User Name = {content}"),
        Err(e) => match e.kind() {
            // we're gracefully handling error by only printing a message (by not causing panic)
            _ => println!("Something went wrong {e:?}")
        },
    }
}


fn read_username_from_file(file_name: &str) -> Result<String, io::Error> {
    let the_file = File::open(file_name);

    let mut username_containing_file = match the_file {
        Ok(file) => file,
        Err(e_message) => return Err(e_message),
        // Err(e_message) => Err(e_message), // will not work here, either panic! or returning the Error (as the enclosing function accept either a String or an Error)
        // the calling the panic! macro will not return the error to the caller function, rather will cause panic here. loosing the ability to handle error later 
        // the return statement used here will cause an early exit, propagating the Error as the return of this function
    };

    let mut user_name = String::new();

    // read the User Name from the file and store in the variable
    let final_result = match username_containing_file.read_to_string(&mut user_name) {
        Ok(_) => Ok(user_name),
        Err(e) => Err(e),
    };

    // Ok(String::new())
    final_result

    // when match statement's result is stored inside of a variable, all of it's arm `return`, ensuring ending the function scope
    // so we can also return either `Ok(String)` or `Err(Message)` directly
    // match username_containing_file.read_to_string(&mut user_name) {
    //     Ok(_) => Ok(user_name),
    //     Err(e) => Err(e),
    // }
}


// using the `?` operator after Result<T, E> instead of match express. Works same, but minimize lots of boilerplate code
fn read_username_from_file_using_question_operator (file_name: &str) -> Result<String, io::Error> {
    let mut the_file = File::open(file_name)?;
    let mut user_name = String::new();
    the_file.read_to_string(&mut user_name)?;
    // we can minimize the code even further by chaining
    // File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(user_name)
}