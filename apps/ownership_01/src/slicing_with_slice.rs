pub fn slicing_with_slice() {
    println!("\n---------------Hello from slicing_with_slice---------------------\n");
    let mut words: String = String::from("Hello World!");
    let first_word = &words[0..5];
    let second_word = &words[5..(words.len() - 1)];
    println!("first_word = {first_word} & second_word = {second_word}");

    let full_word = all_word(&words);
    println!("full_word = {full_word}");

    println!("\n-----------fn first_word -------------\n");
    let first_word_1 = first_word_from_str(&words);
    let first_word_2 = first_word_from_string(&words);
    // words.clear(); // if uncommented, this will throw error, as immutable reference is used later
    println!("{first_word_1} and {first_word_2}");
    words.clear(); // this will work, as no immutable reference is used afterwards


    println!("\n---------------Slice With Array---------------\n");
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &a[1..3];
    assert_eq!(slice, &[2, 3]);
    println!("{:?}", slice); // [2,3]

    println!("\n------------exploring tuple---------------\n");
    exploring_tuple_ref();
}

// experimenting: when a borrowed parameter is passed, then a function can only return a borrowed value
// otherwise, rust compiler will flag this as error, as it will create a dangling pointer
fn all_word(s: &String) -> &str {
    // &s // only `s` will also work
    &s[..] // but here only `s[..]` will not work as `&s[..]` is the actual slice type signature
}

fn first_word_from_string(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index];
        }
    }
    
    &s[..]
}

// this is more convenient than the `first_wrod_from_string` function
// as it can take both a &str or a converted `&String[..]`
fn first_word_from_str(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index];
        }
    }
    &s[..]
}

/**
 * experimenting with destructuring from a tuple
 * using `&` where applicable, we can get a value rather than a borrowed reference
 */
fn exploring_tuple_ref() {
    let x = 10;
    let y = 20;
    let z = 30;

    let tup = (x, &y, &z);
    let (xd, &yd, zd) = tup;
    println!("destructed xd = {xd} and yd = {yd} and zd = {zd}");
    /**
     * note: while destructuring from tuple, prefixing with `&` will 
     */

    let num_array = [1, 2, 3, 4, 5, 6, 7];

    for element in num_array {
        println!("printing {element}");
    }

    // with both index and element
    for (index, &element) in num_array.iter().enumerate() {
        println!("num_array index: {index} and element: {element}")
    }
}