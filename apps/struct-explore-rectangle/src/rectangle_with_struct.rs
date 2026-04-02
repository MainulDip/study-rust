pub fn rectangle_struct() {
    println!("\n-----------------Rectangle with struct-------------------\n");
    let rect = Rectangle{height: 100, width: 100};
    println!("Total area of the rectangle is {}", calculate_area(&rect));
}


fn calculate_area(rec: &Rectangle) -> u32 {
    rec.height * rec.width
}

struct Rectangle {
    height: u32,
    width: u32
}