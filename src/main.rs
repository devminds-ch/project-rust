//! # Rust Training Project
//!
//! A command-line application for basic mathematical calculations.
//!
//! This project demonstrates Rust fundamentals and is used for DevOps CI/CD training
//! by devminds GmbH. The application provides a CLI interface to perform mathematical
//! operations on floating-point numbers.

mod calculate;

use clap::{Parser, Subcommand};

/// Command-line interface structure for the Rust training project.
///
/// This struct defines the main CLI interface using the `clap` crate.
/// It requires a subcommand to be specified when running the application.
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

/// Available commands for the CLI application.
///
/// This enum defines all the subcommands that can be executed by the application.
/// Currently supports mathematical sum operations.
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

/// Main entry point of the application.
///
/// Parses command-line arguments and executes the appropriate command.
/// Currently handles the Sum command to add two floating-point numbers.
fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Sum { a, b } => {
            let result = calculate::sum(*a, *b);
            println!("The sum of {} and {} is {}", *a, *b, result);
        }
    }
}
