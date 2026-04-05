pub fn structs_method_explore() {
    println!("\n------------Struct's Method Exploring----------------\n");
    let rect = Rectangle { height: 100, width: 100};
    println!("Rectangles area is = {}, using method", rect.area());
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// we can have multiple `impl` blocks
impl Rectangle {
    fn is_width_valid(&self) -> bool {
        if self.width > 0 { true } else { false } // rust doesn't have `condition ? exp : exp` like ternary syntax
    }

    // we can have same name as prop
    fn width(&self) -> u32 {
        // get the width of the Rectangle
        if self.width > 0 { self.area() / self.height } else { 0 }      
    }
}

