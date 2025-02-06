use std::path::PathBuf;

use brainfuck::{Executor, IntermediateRepresentation, Lexer};
use clap::{Parser, ValueEnum};

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
    Tokenize,
    IR,
    Run,
}

fn main() {
    let args = Cli::parse();

    let content = if let Some(file) = args.file {
        std::fs::read_to_string(file).expect("failed to read file")
    } else {
        args.program
            .expect("either 'file' or 'program' must be provided")
    };

    match args.command {
        Commands::Tokenize => {
            let lexer = Lexer::new(content.chars());
            for token in lexer {
                println!("{token}");
            }
        }
        Commands::IR => {
            let lexer = Lexer::new(content.chars());
            let ir = IntermediateRepresentation::new(lexer.peekable());
            for instruction in ir {
                let instruction = instruction.unwrap();
                println!("{instruction}");
            }
        }
        Commands::Run => {
            let lexer = Lexer::new(content.chars());
            let ir = IntermediateRepresentation::new(lexer.peekable());
            let mut executor = Executor::default();
            for instruction in ir {
                let instruction = instruction.unwrap();
                executor.execute(&instruction).unwrap();
            }
        }
    }
}
