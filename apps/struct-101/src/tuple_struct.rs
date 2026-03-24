pub fn tuple_struct_call() {
    println!("\n--------tuple struct---------\n");
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    let Point(x, _y, _z) = _origin;
    println!("Color props are {}, {}, {} and point.x is {}", _black.0, _black.1, _black.2, x );
}


struct Color(i32, i32, i32);
struct Point(i32, i32, i32);