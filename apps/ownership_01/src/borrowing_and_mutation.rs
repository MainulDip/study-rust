pub fn borrowing_and_mutation() {
    println!("----------------------Hello from borrowing and mutation--------------------\n");

    let mut s1 = String::from("Hello");
    let s2 = change(&mut s1); // `&mut argument` for mutable borrowed reference 
    // if s1 is not mutable, then mutable borrowing will not work and throw `cannot borrow `s1` as mutable, as it is not declared as mutable`
    println!("s1 = {s1} and s2 = {s2}");
}

// signature for mutable reference (mutable borrowing) is `(param: &mut Type)`
// note: mutable reference will only work if the passed down variable/argument itself is mutable
fn change(str: &mut String) -> String {
    str.push_str(" World!");
    str.to_string()
}


// Dangling reference
// this function will not work
// fn danger() -> &String {
//     let some_string = String::from("Hello World!");
//     &some_string
// }
