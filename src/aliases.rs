use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fs::OpenOptions;
use std::io::Write;
use regex::*;

pub fn definealias(alias: String, filepath: String) {
    let alias = "-<<<>  ".to_owned() + &alias + ": " + &filepath + "\n";
    let mut aliasfile = OpenOptions::new()
        .read(true)
        .append(true)
        .open("crowfile")
        .unwrap();
    aliasfile.write_all(alias.as_bytes())
        .expect("Could not write to file");
}

pub fn openalias(alias: String) {
    let srchterm = "-<<<>  ".to_owned() + &alias + ": ";
    let aliasfile = BufReader::new(File::open("crowfile").unwrap());
        for line in aliasfile.lines() {
            match line {
                Ok(line) => if line.starts_with(&srchterm) {
                    let path: Vec<_> = line.split(": ").collect();
                    let file = &path[1];
                    println!("{}", file);
                },
                Err(e) => panic!("Error reading file: {}", e)
            }
        
        }
}
