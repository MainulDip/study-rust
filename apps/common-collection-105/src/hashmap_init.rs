use std::collections::HashMap;

pub fn hashmap_intro() {
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