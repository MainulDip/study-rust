mod rectangle_without_struct;
mod rectangle_with_tuple;
mod rectangle_with_struct;
mod struct_debug_print;
mod structs_methods;

use rectangle_without_struct::rectangle_001;
use rectangle_with_tuple::rectangle_tuple;
use rectangle_with_struct::rectangle_struct;
use struct_debug_print::struct_debug;
use structs_methods::structs_method_explore;

fn main() {
    rectangle_001();
    rectangle_tuple();
    rectangle_struct();
    struct_debug();
    structs_method_explore();
}