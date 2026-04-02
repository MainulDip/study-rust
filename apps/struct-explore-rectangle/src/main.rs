mod rectangle_without_struct;
mod rectangle_with_tuple;
mod rectangle_with_struct;
mod struct_debug_print;

use rectangle_without_struct::rectangle_001;
use rectangle_with_tuple::rectangle_tuple;
use rectangle_with_struct::rectangle_struct;
use struct_debug_print::struct_debug;

fn main() {
    rectangle_001();
    rectangle_tuple();
    rectangle_struct();
    struct_debug();
}