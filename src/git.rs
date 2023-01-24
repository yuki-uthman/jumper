use crate::error::Error;
use std::process::Command;

#[derive(PartialEq)]
pub enum Direction {
    Forward,
    Backward,
}

pub struct Git {
    pub branch: String,
    pub head: String,
    pub log: Vec<String>,
}

impl Git {
    pub fn new(branch: String) -> Self {
        let head = get_head();
        let log = get_log(&branch).expect("failed to get the current branch name");

        Self { branch, head, log }
    }

    pub fn change_log(&self, file: &str) -> Vec<String> {
        let output = Command::new("git")
            .arg("log")
            .arg(&self.branch.as_str())
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

    pub fn jump_to_commit(&self, direction: Direction) -> Result<(), Error> {
        let mut log = get_log(&self.branch)?;
        if direction == Direction::Backward {
            log.reverse();
        }

        let head = get_head();

        let index = log.iter().position(|r| r == &head).unwrap();
        if index == log.len() - 1 {
            if direction == Direction::Backward {
                return Err(Error::FirstCommit);
            } else {
                return Err(Error::LastCommit);
            }
        }

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
}

fn get_head() -> String {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("HEAD")
        .output()
        .expect("failed to get the current branch name");
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}

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
