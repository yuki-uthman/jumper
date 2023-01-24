use clap::{Parser, Subcommand};
use std::process::Command;

mod error;
mod git;

use error::Error;
use git::{Direction, Git};

fn get_log(branch: &str) -> Result<Vec<String>, Error> {
    let output = Command::new("git")
        .arg("log")
        .arg("--reverse")
        .arg("--pretty=format:%H")
        .arg(branch)
        .output()
        .expect("failed to get the current branch name");

    if !output.status.success() {
        return Err(Error::GitLogEmpty(branch.to_string()));
    }

    let log = std::str::from_utf8(&output.stdout)
        .expect("failed to convert commits output to strings")
        .trim()
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    if log.first().unwrap() == "" {
        panic!("Woops, no commits found for the branch: {}", branch);
    }

    Ok(log)
}

fn get_change_log(branch: &str, file: &str) -> Vec<String> {
    let output = Command::new("git")
        .arg("log")
        .arg(branch)
        .arg("--reverse")
        .arg("--pretty=format:%H")
        .arg("--follow")
        .arg("--")
        .arg(file)
        .output()
        .expect("failed to get the current branch name");

    let log = std::str::from_utf8(&output.stdout)
        .expect("failed to convert commits output to strings")
        .trim()
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    log
}

fn get_head() -> String {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("HEAD")
        .output()
        .expect("failed to get the current branch name");
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}

fn jump_to_change(direction: Direction, branch: &str, file: &str) -> Result<(), Error> {
    let mut master_log = get_log(branch)?;
    if direction == Direction::Backward {
        master_log.reverse();
    }
    let head = get_head();
    let index = master_log.iter().position(|r| r == &head).unwrap();

    let change_log = get_change_log(branch, file);

    // find the next commit that changed the file
    let next_commit = master_log.iter().skip(index + 1).find(|master_commit| {
        // skip if the commit is not in the change log
        if !change_log.contains(master_commit) {
            false
        } else {
            let found = change_log
                .iter()
                .find(|change_commit| change_commit == master_commit);
            match found {
                Some(_) => true,
                None => false,
            }
        }
    });

    // checkout next commit
    match next_commit {
        Some(commit) => {
            let output = Command::new("git")
                .arg("checkout")
                .arg(commit)
                .output()
                .expect("failed to checkout the next commit");

            let stderr = String::from_utf8(output.stderr).unwrap().trim().to_string();
            println!("{}", stderr);
            Ok(())
        }
        None => Err(Error::NoMoreChanges(file.to_string())),
    }
}

#[derive(Parser)]
struct Cli {
    #[arg(short, long, default_value = "master")]
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
}

fn main() {
    let cli = Cli::parse();

    let branch = cli.branch.unwrap();

    let git = Git::new(branch.clone());

    match &cli.command {
        Commands::Next { path } => match path {
            Some(path) => match jump_to_change(Direction::Forward, &branch, path) {
                Ok(_) => {}
                Err(e) => {
                    println!("{}", e);
                }
            },
            None => match git.jump_to_commit(Direction::Forward) {
                Ok(_) => {}
                Err(e) => {
                    println!("{}", e);
                }
            },
        },
        Commands::Prev { path } => match path {
            Some(path) => match jump_to_change(Direction::Backward, &branch, path) {
                Ok(_) => {}
                Err(e) => {
                    println!("{}", e);
                }
            },
            None => match git.jump_to_commit(Direction::Backward) {
                Ok(_) => {}
                Err(e) => {
                    println!("{}", e);
                }
            },
        },
    }
}
