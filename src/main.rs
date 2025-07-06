mod calculate;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    version,
    about = "Rust training project by devminds GmbH",
    long_about = r#"Rust training project by devminds GmbH
This Rust project is used for DevOps CI/CD trainings.
The project contains an application providing a CLI to calculate the sum of two numbers."#,
    subcommand_required = true
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show the sum of two numbers on the console
    Sum {
        /// First value to sum
        a: f64,
        /// Second value to sum
        b: f64,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Sum { a, b } => {
            calculate::sum(*a, *b);
            println!("The sum of {} and {} is {}", *a, *b, calculate::sum(*a, *b));
        }
    }
}
