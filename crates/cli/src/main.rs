use clap::{Parser, Subcommand};
use {{project-name | snake_case}}_core::add;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds two numbers
    Add {
        a: usize,
        b: usize,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { a, b } => {
            println!("Result: {}", add(*a, *b));
        }
    }
}
