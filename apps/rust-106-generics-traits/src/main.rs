mod generic_function_intro;
mod generic_struct_intro;
mod fun_ascii_character;
mod generic_struct_method;

use generic_function_intro::generic_function_introduction;
use generic_struct_intro::generic_struct_introduction;
use fun_ascii_character::print_fun_ascii;
use generic_struct_method::call_generic_struct_method;

fn main() {
    print_fun_ascii();
    generic_function_introduction();
    generic_struct_introduction();
    call_generic_struct_method();
}