#![allow(unused)]
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use std::io::{self, BufRead};
use std::path::Path;

fn write() -> std::io::Result<()> {
    fs::create_dir_all("./temp")?;
    let mut file = File::create("./temp/foo.txt")?;
    file.write_all(b"Hello, world!")?;
    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn handle_read_lines() -> std::io::Result<()>{
    if let Ok(lines) = read_lines("./config") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(item) = line {
                println!("{}", item);
            }
        }
    }
    Ok(())
}

fn bash() -> std::io::Result<()> {
    Command::new("bash")
    .args(&["-c", "echo hello"])
    .spawn()
    .expect("echo command failed to start");

    Ok(())
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
    bash();
    write();
    handle_read_lines();

    Ok(())
}
