mod recovering_error_with_result;
mod propagating_error;

use recovering_error_with_result::recover_error_one;
use propagating_error::propagating_error_fn;

fn main() {
    recover_error_one();
    // println!("Bismillah, Hello, world!");
    // panic!("crash and burn");

    propagating_error_fn();
}
