mod multiple_types_with_enum;

use std::vec;

fn main() {
    println!(" Playing with vector 101 ------------------------- ");

    let mut empty_vector: Vec<i32> = Vec::new();
    // adding new elements by `push`
    empty_vector.push(1);
    empty_vector.push(2);
    empty_vector.push(3);

    let mut vector_with_initial_value = vec![1, 2, 3];
    vector_with_initial_value.push(4);

    let third: &i32 = &vector_with_initial_value[2]; // if index is out-of-bound, the program will panic and exit, here it isn't
    println!("The third element is {third}");

    let option_type_third = vector_with_initial_value.get(2);
    match option_type_third {
        Some(value) => println!("option_type_third is {value}"),
        None => println!("option_type_third doesn't exists"),
    }

    // vector iteration
    let mut vec_iter = vec![1, 2, 3];
    for i in &vec_iter { // here without `&` will also do the loop, but the ownership will be moved
        println!("i = {i} ");
    }
    println!("now the vec_iter = {:?}", vec_iter); // it works as we're using &vec_iter not `vec_iter` directly in the loop

    for i in &mut vec_iter {
        *i += 10; // we need to deference for further assignment in this case
        println!("i after transformation is = {i}");
    }
    println!("vec_iter still valid as we've not moved the ownership yet, proof = {:?}", vec_iter); // but it had been modified by the previous for-in loop to `[11, 12, 13]`

    // Same borrowing rules applied here, We cannot read an immutable borrowed property after it is also been mutably borrowed
    let mut some_vector = vec![1, 2, 3];
    let immutable_borrow_vec_element = &some_vector[2];
    some_vector.push(4);
    // println!("immutable_borrow_vec_element = {immutable_borrow_vec_element}"); // will throw error, not possible to read after it had been modified as borrowed rules applies
    // aka, you cannot read old variable after it's been borrowed mutably.
}
