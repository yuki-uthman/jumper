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

fn main() {
    // git branch name
    let branch = get_branch();
    println!("{}", branch);

    // git log
    let log = get_log();
    println!("{:#?}", log);

    // accept command line argument
    let path = std::env::args().nth(1).expect("failed to get path");
    let changes = changes_for_file(&path);
    println!("{:#?}", changes);

    // get current commit
    let head = get_head();
    println!("HEAD => {}", head);
}
