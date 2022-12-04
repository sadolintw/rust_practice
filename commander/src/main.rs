#![allow(unused)]
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

fn write() -> std::io::Result<()> {
    fs::create_dir_all("./temp")?;
    let mut file = File::create("./temp/foo.txt")?;
    file.write_all(b"Hello, world!")?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    Command::new("bash")
        .args(&["-c", "echo hello"])
        .spawn()
        .expect("echo command failed to start");
    write();
    Ok(())
}
