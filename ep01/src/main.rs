use aoc::read_lines;
use clap::{Parser, Subcommand};

mod ep7;

/// A fictional versioning CLI
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "aoc")]
#[command(about = "The Advent of code CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Clones repos
    #[command(arg_required_else_help = true)]
    Answer {
        /// The remote to clone
        day: String,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Answer { day } => {
            let mut data = vec![];

            if let Ok(lines) = read_lines(format!("{}.txt", day)) {
                for line in lines {
                    if let Ok(val) = line {
                        data.push(val)
                    }
                }
            }

            let answer = ep7::exec(data);

            println!("Answer for day: {}\n{}", day, answer);
        }
    }
}
