use std::process::Command;

fn main() {
    // git branch name
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output()
        .expect("failed to execute process");

    let branch = std::str::from_utf8(&output.stdout).unwrap().trim();
    println!("{}", branch);

    // git log --reverse --pretty=%H master
    let output = Command::new("git")
        .arg("log")
        .arg("--reverse")
        .arg("--pretty=%H")
        .arg(branch)
        .output()
        .expect("failed to execute process");

    let commits = std::str::from_utf8(&output.stdout)
        .expect("Failed to convert output to strings")
        .trim()
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    // print output
    println!("{:?}", commits);
}
