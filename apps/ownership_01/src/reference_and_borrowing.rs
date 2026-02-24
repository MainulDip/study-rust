pub fn reference_and_borrowing() {
    let s1 = String::from("Hello World!");
    let s_len = calculate_length(&s1);
    println!("s1 = \"{s1}\" and length is sLen = {s_len}");
    // s1 is still valid as it had been borrowed using `&` and not used directly
}


fn calculate_length(str: &String) -> usize {
    str.len()
}
