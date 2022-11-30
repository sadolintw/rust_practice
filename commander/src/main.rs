use std::process::Command;

fn main() {
    println!("Hello, world!");
    Command::new("bash").args(&["-c", "echo hello\n"]).spawn().expect("echo command failed to start");
}