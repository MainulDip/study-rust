pub fn no_slice_find_word() {
    println!("Hello from no slice find word");
}


// In idiomatic Rust, functions do not take ownership of their arguments unless they need to, and the reasons for that will become clear as we keep going
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' '  {
            return index;
        }
    }

    s.len()
}
