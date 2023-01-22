use std::process::Command;

fn get_branch() -> String {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output()
        .expect("failed to get the current branch name");
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}

fn get_log() -> Vec<String> {
    let output = Command::new("git")
        .arg("log")
        .arg("--reverse")
        .arg("--pretty=format:%H")
        .arg("master")
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

fn changes_for_file(file: &str) -> Vec<String> {
    let output = Command::new("git")
        .arg("log")
        .arg("--reverse")
        .arg("--pretty=format:%H")
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

fn jump_to_next_commit() {
    let log = get_log();
    let head = get_head();
    let index = log.iter().position(|r| r == &head).unwrap();
    let next_commit = &log[index + 1];
    let output = Command::new("git")
        .arg("checkout")
        .arg(next_commit)
        .output()
        .expect("failed to get checkout the next commit");

    let stderr = String::from_utf8(output.stderr).unwrap().trim().to_string();
    println!("{}", stderr);
}

fn main() {
    // get current commit
    let head = get_head();
    println!("HEAD => {}", head);

    // git log
    let log = get_log();
    println!("{:#?}", log);

    // check if argument is given
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("No file given");
        // get index of the head commit in the log
        let index = log.iter().position(|r| r == &head).unwrap();

        // check if it is the last commit
        if index == log.len() - 1 {
            println!("HEAD is the last commit");
        } else {
            jump_to_next_commit();
        }
    } else {
        let file = &args[1];
        println!("File: {}", file);

        // git log for file
        let log = changes_for_file(file);
        println!("{:#?}", log);
    }
}
