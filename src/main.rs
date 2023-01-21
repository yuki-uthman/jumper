use std::process::Command;

fn main() {
    // git log --reverse --pretty=%H master
    let output = Command::new("git")
        .arg("log")
        .arg("--reverse")
        .arg("--pretty=%H")
        .arg("master")
        .output()
        .expect("failed to execute process");

    // print output
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
