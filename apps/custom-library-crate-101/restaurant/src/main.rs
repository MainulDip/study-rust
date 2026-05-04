// use rand::Rng;

// use rand::Rng; // we're only bringing the Rng trait's scope
// we can also create shortcut (& import) both `thread_rng` and `gen_range` separately, and use the shortcut, it's the same
use rand::{thread_rng, Rng};

fn main() {
    // add_to_waitlist();

    // testing external package usages
    // here, we're directly accessing the `rand` crate's thread_rnd function and calling gen_range (which comes with the Rng trait, we imported/shortcut earlier)
    let secret_number = rand::thread_rng().gen_range(1..10);

    // here we're using the shortcut for `thread_rng`
    let secret_number_2 = thread_rng().gen_range(1..=100);

    // directly calling `Rng` trait's function, without using any `use` block to import
    let secret_number_3 = rand::Rng::gen_range(&mut rand::thread_rng(), 1..=100);

    // using `use rand::{thread_rng, Rng}`, but differently, starting from `Rng::`
    let secret_number = Rng::gen_range(&mut thread_rng(), 1..=100);

    
    println!("secret_number_1 = {secret_number} and secret_number_2 = {secret_number_2}");
}