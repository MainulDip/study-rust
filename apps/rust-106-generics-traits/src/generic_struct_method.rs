pub fn call_generic_struct_method() {
    println!("\n\n--------------Generic Struct Method------------------------\n\n");
    let container_1 = Container { x: 7, y: 7 };
    let container_2 = Container { x: "Hello", y: "World" };
    let final_container = container_1.mixup(container_2);
    println!("Mixed Up Container is {final_container:?}");
}


#[derive(Debug)]
struct Container<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Container<X1, Y1> {
    fn mixup<X2, Y2>(self, other_container: Container<X2, Y2>) -> Container<X1, Y2> {
        Container { x: self.x, y: other_container.y }
    }
}
