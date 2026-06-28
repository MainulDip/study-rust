pub fn generic_struct_introduction() /* -> Point<i32, f64> */ {
    println!("\n\n------------Generic Struct Playground-------------------\n\n");
    let point = Point{ width: 100, height: 100.4 };
    println!("point = {point:?}");
    println!("point width = {} and height = {}", point.width, point.height);
    /* point */
}


// create a Generic Point struct, with two different Generic types
#[derive(Debug)]
struct Point<T, U> {
    width: T,
    height: U
} 

// Note: For same file, private fields can be accessed (unless behind a module)
// But to access the struct from another file, the struct needs to be public (pub)
// Also, to access the field outside, it needs to be public as well
// Like, if the public facing function return a Point object, and we capture that from main function, it needs to modify all those private fields to public
/*
 * let mut sth = generic_struct_introduction(); // will not work unless we make the Point struct public
 * sth.width = 200.4; // won't work unless we make the width public
 */