pub fn struct_debug() {
    println!("\n--------------------Struct Debug Printing-----------------------\n");
    calculate();
    let rect = Rectangle{height: 100, width: 100};
    println!("rect is {rect:?}");
    println!("rect is {rect:#?}"); // prettier than the `:?` version


    let scale = 2;
    let rect2 = Rectangle {
        height: dbg!(30 * scale),
        width: 30
    };
    println!("\n---------dbg!(&v)---------\n");
    dbg!(&rect2);
}


fn calculate() -> String {
    String::from("Calculating")
}

#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32
}