use std::{fs, io};
use std::path::Path;
use crate::algorithms::two_sum::algo::profile_algo_gradual;
use crate::{generate_interactive_plot, generate_terminal_plot, solve_two_sum};
use clap::{Parser, Subcommand};

/// AlgoProfile - Algorithm Performance Profiling Tool
///
/// Profile and visualize algorithm time and memory complexity with interactive plots
#[derive(Parser)]
#[command(name = "algoprofile")]
#[command(version = "0.1.0")]
#[command(about = "Algorithm profiling and visualization tool", long_about = None)]
#[command(after_help = "EXAMPLES:\n  \
    algoprofile list                    # List all available algorithms\n  \
    algoprofile run two_sum             # Run a specific algorithm\n  \
    algoprofile profile two_sum         # Profile algorithm with visualization\n  \
    algoprofile profile two_sum --interactive  # Launch interactive plot\n")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all available algorithms
    List,

    /// Run a specific algorithm with sample data
    Run {
        /// Name of the algorithm to run
        algorithm: String,
    },

    /// Profile an algorithm and visualize performance
    Profile {
        /// Name of the algorithm to profile
        algorithm: String,

        /// Launch interactive plot immediately after profiling
        #[arg(short, long)]
        interactive: bool,
    },
}
pub fn start_cli(){
    let cli = Cli::parse();

    match cli.command {
        Commands::List => {
            if let Err(e) = list_algorithms() {
                eprintln!("Error listing algorithms: {}", e);
            }
        }
        Commands::Run { algorithm } => {
            run_algorithm(&algorithm);
        }
        Commands::Profile { algorithm, interactive } => {
            profile_algorithm(&algorithm, interactive);
        }
    }
}

fn list_algorithms() -> io::Result<()> {
    println!("\n📚 Available Algorithms:\n");
    println!("{:<20} {:<15} {:<40}", "Name", "Complexity", "Description");
    println!("{}", "─".repeat(75));

    let path = Path::new("./algoprofile/src/algorithms");

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            let name = entry.file_name();
            let name_str = name.to_str().unwrap();

            // Hardcoded info for now - can be made dynamic later
            match name_str {
                "two_sum" => println!(
                    "{:<20} {:<15} {:<40}",
                    "two_sum", "O(n)", "Find two numbers that sum to target"
                ),
                _ => println!("{:<20} {:<15} {:<40}", name_str, "Unknown", "Algorithm"),
            }
        }
    }
    println!();
    Ok(())
}

fn run_algorithm(algo_name: &str) {
    println!("\n🚀 Running algorithm: {}\n", algo_name);

    match algo_name {
        "two_sum" => {
            let nums = vec![2, 7, 11, 15];
            let target = 9;
            let result = solve_two_sum(nums.clone(), target);
            println!("Input: {:?}, Target: {}", nums, target);
            println!("Result: {:?}\n", result);
        }
        _ => println!(
            "❌ Algorithm '{}' not found. Use 'list' to see available algorithms.\n",
            algo_name
        ),
    }
}

fn profile_algorithm(algo_name: &str, interactive: bool) {
    println!("\n📊 Profiling algorithm: {}\n", algo_name);

    match algo_name {
        "two_sum" => {
            let gradual_data = profile_algo_gradual();
            generate_terminal_plot(&gradual_data);

            if interactive {
                match generate_interactive_plot(gradual_data) {
                    Ok(_) => println!("\n✓ Interactive plot closed"),
                    Err(e) => eprintln!("\n✗ Error in interactive plot: {}", e),
                }
            }
        }
        _ => println!(
            "❌ Algorithm '{}' not found. Use 'list' to see available algorithms.\n",
            algo_name
        ),
    }
}