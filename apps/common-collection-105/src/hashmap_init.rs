use std::collections::HashMap;

pub fn hashmap_intro() {
    hashmap_init_using_form();

    hashmap_mutable_examples();

    
    println!("\n\nHashMap replace the old value with new one");
    hashmap_replace_old_value(200);

    println!("\n\n Updating Hashmap by combining the oldValue");
    hashmap_increment_value();

    println!("\n\nHashMap keep the old value if less than 100");
    hashmap_keep_old(90);
}

// common usages of HashMap::form
fn hashmap_init_using_form() {
    // using HashMap::from fn
    let hash_map_all_numbe: HashMap<i32, i32> = HashMap::from([(10, 10)]); // funny (key as number), here it will accept list of tuple
    let hash_map_str: HashMap<&str, i32> = HashMap::from([("a", 10)]);
    let hash_map_string: HashMap<String, i32> = HashMap::from([(String::from("A"), 10)]);

    // using collect with vector or iterator
    let fruits_vector: Vec<(&str, i32)> = vec![("Apple", 10),("Orange", 20),("Mango", 30)];
    // convert the vector into iterator and collect to make a hashmap
    let fruits_hashmap: HashMap<&str, i32> = fruits_vector.into_iter().collect(); // into_inter() will work, but not iter(), as iter() will return a borrowed type

    // merging 2 separate vector as hashmap using `zip()` and `collect()`
    // when keys and values are in separate collections, .zip() fn can be used to pair them up into tuples and then call .collect() to create hashmap
    let keys_vec = vec!["Apple", "Orange", "Mango"];
    let values_vec = vec![10, 20, 30];

    let zipped_fruits_hashmap: HashMap<&str, i32> = keys_vec.into_iter().zip(values_vec).collect(); // type needs to be explicit
}

// HashMap mutable initialization

fn hashmap_mutable_examples() {
    println!("\n\nHashMap mutable examples\n\n");

    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Cricket"), 370);
    scores.insert(String::from("Football"), 4);
    // when accessed by key using the `get` function, it returns an Option<T> (either Some(T) or None), so we need to unwrap that
    println!("Scores for Cricket is {:?} and for Football is {:?}", scores.get(&String::from("Cricket")).expect("msg"), scores.get(&String::from("Football")).expect("nothing matches as the supplied key"));

    // iterating over a HashMap
    for (key, value) in scores {
        println!("Key {key} and value {value}");
    }
}

// - overwrite/replace the oldValue with newValue | `insert` with same key
fn hashmap_replace_old_value(new_balance: i32) {
    let mut users_balance: HashMap<&str, i32> = HashMap::new();
    let userId = "1233";
    let minimum_balance = 100;
    users_balance.insert(userId, minimum_balance);

    // update the new value
    users_balance.insert(userId, new_balance);

    println!("{users_balance:?}");
}

// - add the new value, if the key doesn't already have a value | `entry(Key).or_insert(Value)`
// - combine the old value and new value 
fn hashmap_increment_value() {
    let mut hashmap: HashMap<&str, i32> = HashMap::new();
    let text = "hello world wonderful world";

    for word in text.split_whitespace() {
        let count = hashmap.entry(word).or_insert(0);
        *count += 1;
    }

    println!("hashmap = {:?}", hashmap);
}

// - keep the old value and discard the new value by checking if newValue is less than 100
fn hashmap_keep_old(new_balance: i32) {
    let mut users_balance: HashMap<&str, i32> = HashMap::new();
    let userId = "1233";
    let minimum_balance = 100;
    users_balance.insert(userId, minimum_balance);

    let user_handle = users_balance.entry(userId).or_insert(0);
    if new_balance > minimum_balance {
        *user_handle = new_balance;
    }

    println!("User id = {} and User's balance is {}", userId, user_handle);

}
