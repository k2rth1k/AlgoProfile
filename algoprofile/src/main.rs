use crate::algorithms::two_sum::algo::two_sum::algo_profile;
use crate::data_generation::lib::utils::new_numeric_vector_with_size_and_range;
use crate::ui::main_ui::App;
use std::io;

mod algorithms;
mod cli;
pub mod data_generation;
pub mod ui;

fn main() -> io::Result<()> {
    // Test algorithm profiling
    let res = algo_profile(0, 0, 0);
    println!("{:?}", res);

    println!(
        "{:?}",
        new_numeric_vector_with_size_and_range::<i32>(10, 0, 12)
    );

    ratatui::run(|terminal| App::default().run(terminal))
}
