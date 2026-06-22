pub fn generics_introduction() {
    println!("Introducing Generics 101");

    let number_list_array = [1, 2, 3];
    let largest_number_from_array = largest_i32(&number_list_array);
    println!("Largest number from the array {number_list_array:?} is {largest_number_from_array}");

    let number_list_vector = vec![1, 2, 3, 4, 5, 6, 7];
    let largest_number_from_vector = largest_i32(&number_list_vector);
    println!("Largest number from the vector {number_list_vector:?} is {largest_number_from_vector}");

    let char_vector = vec!['a', 'b', 'c'];
    let largest_char_from_char_vector = largest_char(&char_vector);
    println!("Largest char from the vector {char_vector:?} is {largest_char_from_char_vector}");


    println!("\n\n Using generics to find largest in both numbers and characters list");
    println!("Largest number from the vector {number_list_vector:?} using generics is {}", largest_findings_generics(&number_list_vector));
    println!("Largest char from the vector {char_vector:?} using generics is {}", largest_findings_generics(&char_vector));

}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// implementing generics for finding largest number or char
fn largest_findings_generics<T: std::cmp::PartialOrd> (list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}