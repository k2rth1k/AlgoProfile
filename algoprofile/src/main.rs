use crate::algorithms::two_sum::algo::two_sum::algo_profile;
use crate::data_generation::lib::utils::new_numeric_vector_with_size_and_range;

mod algorithms;
mod cli;
pub mod data_generation;

fn main() {
    let res = algo_profile(0, 0, 0);

    println!("{:?}", res);

    println!(
        "{:?}",
        new_numeric_vector_with_size_and_range::<i32>(10, 0, 12)
    );
}
