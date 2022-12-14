#![allow(unused)]
#[macro_use]
extern crate lazy_static;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::Command;
use std::sync::Mutex;
use std::sync::MutexGuard;

lazy_static! {
    static ref HASHMAP: Mutex<HashMap<&'static str, &'static str>> = {
        let mut m = HashMap::new();
        m.insert("0", "foo");
        m.insert("1", "bar");
        m.insert("2", "baz");
        Mutex::new(m)
    };
}

fn write() -> std::io::Result<()> {
    println!("write");
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

fn handle_read_lines() -> std::io::Result<()> {
    println!("handle_read_lines");
    if let Ok(lines) = read_lines("./config") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(item) = line {
                if !item.starts_with("#") {
                    let vec: Vec<&str> = item.split("=").collect();
                    println!("{:?} {} {}", vec, vec[0], vec[1]);
                }
            }
        }
    }
    Ok(())
}

fn bash() -> std::io::Result<()> {
    println!("bash");
    Command::new("sh")
        .arg("-c")
        .arg("echo hello")
        .spawn()?
        .wait()
        // .output()
        .expect("failed to execute process");
    // println!("{}", output.stdout.as_slice())
    Ok(())
}

// fn print_hashmap(map: MutexGuard<HashMap<&'static str, &'static str>>) -> std::io::Result<()> {
fn print_hashmap() -> std::io::Result<()> {
    println!("print_hashmap");
    let mut map = HASHMAP.lock().unwrap();
    for (key, value) in &*map {
        println!("{} / {}", key, value);
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
    print_hashmap();
    let mut map = HASHMAP.lock().unwrap();
    // print_hashmap(map);
    map.insert("3", "hello");
    // map = HASHMAP.lock().unwrap();
    drop(map);
    print_hashmap();
    
    println!("");
    bash();
    println!("");
    write();
    println!("");
    handle_read_lines();
    
    
    Ok(())
}
