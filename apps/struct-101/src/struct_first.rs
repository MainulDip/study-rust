pub fn instantiate_struct_101() {
    let user_1 = User {
        _active: true,
        username: String::from("UserOne"),
        email: String::from("userone@email.com"),
        _sign_in_count: 1
    };

    println!("user_1 name: {} and email is: {}", user_1.username, user_1.email);

    // struct ownership/borrow follow the same principe
    // if the instantiated property is heap stored, it will be moved, so after using anything like that to build a new struct instance
    // the old instance cannot be used afterward (in simple terms)

    let user_2: User = User { _active: true, username: user_1.username, email: user_1.email, _sign_in_count: 1 };
    // now, the user_1 is no longer available
    // println!("user_1 name: {} and email is: {}", user_1.username, user_1.email); // error

    // we can also use the spread syntax like (json) to build a new struct from older struct
    // modified property should come before the spared call
    let user_3: User = User { username: String::from("UserThreeName"), email: String::from("userthree@email.com"), ..user_2  };
    println!("user_3 name: {} and email is: {}", user_3.username, user_3.email);
    // user_2 is still valid, because, we use `_active` and `_sign_in_count` from the user_2, which are stack stored (aka, has copy trait implemented)
    println!("user_2 name: {} and email is: {}", user_2.username, user_2.email);

    // build user using builder function
    let user_4 = user_builder(String::from("UserFourName"), String::from("userforu@email.com"));
    println!("user_4 name: {} and email is: {}", user_4.username, user_4.email);

    // if declared mutable, all properties get the mutability, no way to make partial mutable
    let mut user_5: User = User { _active: true, username: String::from("UserFiveName"), email: String::from("userfive@email.com"), _sign_in_count: 5 };
    println!("user_5 name: {} and email is: {}", user_5.username, user_5.email);
    user_5.email = String::from("newuserfive@email.com");
    println!("user_5's modified email is: {}", user_5.email);

    
    // building another user with the build_2 sending an already instantiate user
    let user_6 = user_builder_2(String::from("UserSixName"), String::from("usersix@email.com"), user_4);
    println!("user_6 name: {} and email is: {}", user_6.username, user_6.email); 

}

struct User {
    _active: bool,
    username: String,
    email: String,
    _sign_in_count: u64
}

// when a struct property and a variable have the same name, we can directly map the variable by only key (without the `key:val` syntax)
fn user_builder(username: String, email: String) -> User {
    User { _active: true, username, email, _sign_in_count: 1 }
}

fn user_builder_2(username: String, email: String, old_user: User) -> User {
    User { username, email, ..old_user }
}