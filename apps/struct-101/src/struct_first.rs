pub fn instantiate_struct_101() {
    let user_1 = User {
        _active: true,
        username: String::from("UserOne"),
        email: String::from("userone@email.com"),
        _sign_in_count: 1
    };

    println!("user_1 name: {} and email is: {}", user_1.username, user_1.email);
}

struct User {
    _active: bool,
    username: String,
    email: String,
    _sign_in_count: u64
}