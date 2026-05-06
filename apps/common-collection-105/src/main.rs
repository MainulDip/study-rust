fn main() {
    println!(" Playing with vector 101 ------------------------- ");

    let mut empty_vector: Vec<i32> = Vec::new();
    // adding new elements by `push`
    empty_vector.push(1);
    empty_vector.push(2);
    empty_vector.push(3);

    let mut vector_with_initial_value = vec![1, 2, 3];
    vector_with_initial_value.push(4);

    let third: &i32 = &vector_with_initial_value[2]; // if index is out-of-bound, the program will panic and exit
    println!("The third element is {third}");

    let option_type_third = vector_with_initial_value.get(2);
    match option_type_third {
        Some(value) => println!("option_type_third is {value}"),
        None => println!("option_type_third doesn't exists"),
    }
}
