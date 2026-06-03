pub fn utf8_string_init() {
    println!("\"দ\" byte size is {:?}", "দ".len()); // 3, this is byte size, not character count. As utf-8 character can be 1 to 4 byte (variable size)
    println!("\"দ\" as byte hex values {:?}", "দ".as_bytes()); // [224, 166, 166]
    println!("\"D\" as character's byte size is {}", 'D'.len_utf8()); // 1, this is byte size, not character count
    println!("\"দ\" chars count is {:?}", "দ".chars().count()); // 1
    println!("\"দ\" byte size is {}", 'দ'.len_utf8()); // 3
}