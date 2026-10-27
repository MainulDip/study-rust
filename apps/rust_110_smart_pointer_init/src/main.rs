mod box_smart_pointer;
mod custom_struct_deref_impl;
mod drop_trait_init;
mod reference_counting_init;

use box_smart_pointer::box_smart_pointer_init;
use custom_struct_deref_impl::custom_struct_deref_trait_implementation;
use drop_trait_init::init_drop_trait;
use reference_counting_init::reference_counting_init;

fn main() {
    box_smart_pointer_init();
    custom_struct_deref_trait_implementation();
    init_drop_trait();
    reference_counting_init();
}
