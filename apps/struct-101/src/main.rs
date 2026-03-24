mod struct_first;
mod tuple_struct;

use struct_first::instantiate_struct_101;
use tuple_struct::tuple_struct_call;
fn main() {
    println!("Hello, world form struct 101");
    instantiate_struct_101();
    tuple_struct_call();
}
