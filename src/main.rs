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

fn main() {
    // git branch name
    let branch = get_branch();
    println!("{}", branch);

    // git log --reverse --pretty=%H master
    let output = Command::new("git")
        .arg("log")
        .arg("--reverse")
        .arg("--pretty=%H")
        .arg(branch)
        .output()
        .expect("failed to execute git log");

    let commits = std::str::from_utf8(&output.stdout)
        .expect("failed to convert commits output to strings")
        .trim()
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    // print output
    println!("{:?}", commits);

    // accept command line argument
    let path = std::env::args().nth(1).expect("failed to get path");

    let output = Command::new("git")
        .arg("log")
        .arg("--pretty=%H")
        .arg("--reverse")
        .arg("--follow")
        .arg("--")
        .arg(path)
        .output()
        .expect("failed to execute git log");

    let changes = std::str::from_utf8(&output.stdout)
        .expect("failed to convert commits output to strings")
        .trim()
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    println!("{:?}", changes);

    // get current commit
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("HEAD")
        .output()
        .expect("failed to get the current commit");

    let head = std::str::from_utf8(&output.stdout).unwrap().trim();
    println!("HEAD => {}", head);
}
