use std::process::Command;

fn main() {
    // run git command
    let output = Command::new("git")
        .arg("status")
        .output()
        .expect("failed to execute process");

    // print output
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
