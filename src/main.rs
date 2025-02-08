use std::path::PathBuf;

use brainfuck::{Executor, InstructionIterator, TokenIterator};
use clap::{Parser, ValueEnum};

/// A Brainfuck interpreter
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(value_enum)]
    command: Commands,
    #[arg(short, long)]
    file: Option<PathBuf>,
    program: Option<String>,
}

#[derive(ValueEnum, Clone, Copy)]
enum Commands {
    /// Transform the program into [`Token`]s.
    Tokenize,
    /// Transform the program into [`Instruction`]s.
    IR,
    /// Parse the program and run it.
    Run,
}

fn main() {
    let args = Cli::parse();

    let program = if let Some(file) = args.file {
        std::fs::read_to_string(file).expect("failed to read file")
    } else {
        args.program
            .expect("either 'file' or 'program' must be provided")
    };

    match args.command {
        Commands::Tokenize => {
            for (_, token) in program.iter_token() {
                println!("{token}");
            }
        }
        Commands::IR => {
            for instruction in program.iter_instruction().map(Result::unwrap) {
                println!("{instruction}");
            }
        }
        Commands::Run => {
            let mut executor = Executor::default();
            for instruction in program.iter_instruction().map(Result::unwrap) {
                executor.execute(instruction).unwrap();
            }
        }
    }
}
