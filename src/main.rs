use std::process::ExitCode;
use clap::{Parser, Subcommand};

mod error;
mod git;

use git::{Direction, Git};

#[derive(Parser)]
struct Cli {
    #[arg(global = true, short, long, default_value = "master")]
    branch: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// to the next commit or if path is given then to the next commit that changed the file
    Next { path: Option<String> },

    /// to the prev commit or if path is given then to the prev commit that changed the file
    Prev { path: Option<String> },

    First,
    Last,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let git = match Git::new(cli.branch.unwrap()) {
        Ok(git) => git,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        }
    };


    match &cli.command {
        Commands::First => {
            match git.jump_to_fist() {
                Ok(_) => ExitCode::SUCCESS,
                Err(e) => {
                    eprintln!("{}", e);
                    ExitCode::FAILURE
                }
            }
        }
        Commands::Last => {
            match git.jump_to_last() {
                Ok(_) => ExitCode::SUCCESS,
                Err(e) => {
                    eprintln!("{}", e);
                    ExitCode::FAILURE
                }
            }
        }
        Commands::Next { path } => match path {
            Some(path) => match git.jump_to_change(Direction::Forward, path) {
                Ok(_) => { ExitCode::SUCCESS  },
                Err(e) => {
                    eprintln!("{}", e);
                    ExitCode::FAILURE
                }
            },
            None => match git.jump_to_commit(Direction::Forward) {
                Ok(_) => { ExitCode::SUCCESS  },
                Err(e) => {
                    eprintln!("{}", e);
                    ExitCode::FAILURE
                }
            },
        },
        Commands::Prev { path } => match path {
            Some(path) => match git.jump_to_change(Direction::Backward, path) {
                Ok(_) => { ExitCode::SUCCESS  },
                Err(e) => {
                    eprintln!("{}", e);
                    ExitCode::FAILURE
                }
            },
            None => match git.jump_to_commit(Direction::Backward) {
                Ok(_) => { ExitCode::SUCCESS  },
                Err(e) => {
                    eprintln!("{}", e);
                    ExitCode::FAILURE
                }
            },
        },
    }
}
