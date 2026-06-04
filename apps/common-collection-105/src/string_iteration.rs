pub fn string_iteration_init() {
    for c in "কাক".chars() {
        println!("{c}");
    }

    println!("\n\nকাক byte length is = {}\n\n", "কাক".len());

    for c in "কাক".as_bytes() {
        // println!("Byte value {c}");
        println!("{c}");
        // 224, 166, 149, 224, 166, 190, 224, 166, 149
    }

    for c in "কাক".chars() {
        println!("{c}");
    }
    // ক, া, ক

    // Slicing, slicing should be done with caution, if the output/rendered byte is not a valid character, the program will panic
    let s = String::from("কাক");
    let string_slice_valid = &s[0..3];
    println!("{string_slice_valid}"); // will print "ক", it's a valid character in bengali language
    // let string_slice_not_valid = &s[0..1]; // will panic as the first byte is not a valid character, it needs 3 byte to form character "ক"
    // println!("{string_slice_not_valid}"); // before reaching this point, the program will panic (to create a variable based on non character slice)
}