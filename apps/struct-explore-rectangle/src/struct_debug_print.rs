pub fn struct_debug() {
    println!("\n--------------------Struct Debug Printing-----------------------\n");
    calculate();
    let rect = Rectangle{height: 100, width: 100};
    println!("rect is {rect:?}");
    println!("rect is {rect:#?}"); // prettier than the `:?` version


    println!("rect printing with :?, so the rect is = {:?}", rect);
    println!("rect printing with :?, so the rect is = {:#?}", rect);


    let scale = 2;
    let rect2 = Rectangle {
        height: dbg!(30 * scale), // dbg! returns ownership of the expression’s value, the width field will get the same value as if we didn’t have the dbg!
        width: 30
    };
    println!("\n---------dbg!(&v)---------\n");
    dbg!(&rect2); // here, we are sending a reference, as we don't want to move the ownership
}


fn calculate() -> String {
    String::from("Calculating")
}


// to print out a struct for debugging, we need to explicitly opt in to make that functionality available
#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32
}