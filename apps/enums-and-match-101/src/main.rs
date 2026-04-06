mod enum_001;
mod associated_values;

use enum_001::enum_def_001;
use associated_values::use_enum_associated_value;

fn main() {
    enum_def_001();
    use_enum_associated_value();
}
