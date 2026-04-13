mod enum_001;
mod associated_values;
mod option_handling_a;

use enum_001::enum_def_001;
use associated_values::use_enum_associated_value;
use option_handling_a::option_handle_a;

fn main() {
    enum_def_001();
    use_enum_associated_value();
    option_handle_a();
}
