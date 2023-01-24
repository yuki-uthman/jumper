use clap::{Parser, Subcommand};
use std::process::Command;

mod error;

use error::Error;

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

fn get_change_log(file: &str) -> Vec<String> {
    let output = Command::new("git")
        .arg("log")
        .arg("master")
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

fn jump_to_previous_commit(branch: &str) {
    let log = get_log(branch).unwrap();
    let head = get_head();

    let index = log.iter().position(|r| r == &head).unwrap();

    if index == 0 {
        println!("Already at the first commit");
        return;
    }

    let prev_commit = &log[index - 1];
    let output = Command::new("git")
        .arg("checkout")
        .arg(prev_commit)
        .output()
        .expect("failed to get checkout the next commit");

    let stderr = String::from_utf8(output.stderr).unwrap().trim().to_string();
    println!("{}", stderr);
}

fn jump_to_next_commit(branch: &str) -> Result<(), Error> {
    let log = get_log(branch)?;
    let head = get_head();

    if is_last_commit(&log, &head) {
        println!("Already at the last commit");
        return Ok(());
    }

    let index = log.iter().position(|r| r == &head).unwrap();
    let next_commit = &log[index + 1];
    let output = Command::new("git")
        .arg("checkout")
        .arg(next_commit)
        .output()
        .expect("failed to get checkout the next commit");

    let stderr = String::from_utf8(output.stderr).unwrap().trim().to_string();
    println!("{}", stderr);
    Ok(())
}

fn is_last_commit(log: &Vec<String>, head: &String) -> bool {
    let index = log.iter().position(|r| r == head).unwrap();
    index == log.len() - 1
}

fn jump_to_prev_change(branch: &str, file: &str) {
    let mut master_log = get_log(branch).unwrap();
    master_log.reverse();

    let head = get_head();
    let index = master_log.iter().position(|r| r == &head).unwrap();

    let change_log = get_change_log(file);
    // println!("Change log: \n{:#?}", change_log);

    // find the next commit that changed the file
    let prev_commit = master_log.iter().skip(index + 1).find(|master_commit| {
        if !change_log.contains(master_commit) {
            false
        } else {
            let found = change_log.iter().find(|change_commit| {
                // println!("{} == {}", master_commit, change_commit);
                change_commit == master_commit
            });
            match found {
                Some(_) => true,
                None => false,
            }
        }
    });

    match prev_commit {
        Some(commit) => {
            let output = Command::new("git")
                .arg("checkout")
                .arg(commit)
                .output()
                .expect("failed to get checkout the next commit");

            let stderr = String::from_utf8(output.stderr).unwrap().trim().to_string();
            println!("{}", stderr);
        }
        None => {
            println!("No more changes for {}", file);
        }
    }
}


fn jump_to_next_change(branch: &str, file: &str) {
    // index of the current commit
    let master_log = get_log(branch).unwrap();
    // println!("Master log: \n{:#?}", master_log);

    let head = get_head();
    let index = master_log.iter().position(|r| r == &head).unwrap();

    // git log for file
    let change_log = get_change_log(file);
    // println!("Change log: \n{:#?}", change_log);

    // find the next commit that changed the file
    let next_commit = master_log.iter().skip(index + 1).find(|master_commit| {
        // skip if the commit is not in the change log
        if !change_log.contains(master_commit) {
            false
        } else {
            let found = change_log.iter().find(|change_commit| {
                // println!("{} == {}", master_commit, change_commit);
                change_commit == master_commit
            });
            match found {
                Some(_) => true,
                None => false,
            }
        }
    });

    // println!("{:#?}", next_commit);

    // checkout next commit
    match next_commit {
        Some(commit) => {
            let output = Command::new("git")
                .arg("checkout")
                .arg(commit)
                .output()
                .expect("failed to get checkout the next commit");

            let stderr = String::from_utf8(output.stderr).unwrap().trim().to_string();
            println!("{}", stderr);
        }
        None => {
            println!("No more changes for {}", file);
        }
    }
}

#[derive(Parser)]
struct Cli {
    #[arg(short, long, default_value = "master")]
    branch: Option<String>,

    path: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Next { path: Option<String> },
    Prev { path: Option<String> },
}

fn main() {
    let cli = Cli::parse();

    let branch = cli.branch.unwrap();

    match &cli.command {
        Commands::Next { path } => match path {
            Some(path) => {
                jump_to_next_change(&branch, path);
            }
            None => {
                match jump_to_next_commit(&branch) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            }
        },
        Commands::Prev { path } => match path {
            Some(path) => {
                jump_to_prev_change(&branch, path);
            }
            None => {
                jump_to_previous_commit(&branch);
            }
        },
    }
}
