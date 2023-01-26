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
    pub fn new(branch: String) -> Result<Self, Error> {
        let head = get_head()?;
        let log = get_log(&branch)?;

        Ok(Self { branch, head, log })
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
        let mut log = self.log.clone();
        if direction == Direction::Backward {
            log.reverse();
        }

        let index = log.iter().position(|r| r == &self.head).unwrap();
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

    pub fn jump_to_change(&self, direction: Direction, file: &str) -> Result<(), Error> {
        let mut log = self.log.clone();
        if direction == Direction::Backward {
            log.reverse();
        }
        let head = self.head.clone();
        let index = log.iter().position(|r| r == &head).unwrap();

        let change_log = self.change_log(file);

        // find the next commit that changed the file
        let next_commit = log.iter().skip(index + 1).find(|commit| {
            // skip if the commit is not in the change log
            if !change_log.contains(commit) {
                false
            } else {
                let found = change_log
                    .iter()
                    .find(|change_commit| commit == change_commit);
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
}

fn get_head() -> Result<String, Error> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("HEAD")
        .output()
        .expect("failed to get the current branch name");
    Ok(String::from_utf8(output.stdout).unwrap().trim().to_string())
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
