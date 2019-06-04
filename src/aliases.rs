use std::fs::OpenOptions;
use std::io::Write;
// use std::fs;
// use std::fs::File;
// use std::io::prelude::*;

pub fn definealias(alias: String, filepath: String) {
    let alias = "-<<<>  ".to_owned() + &alias + ": " + &filepath + "\n";
    let mut aliasfile = OpenOptions::new()
        .read(true)
        // .write(true)
        .append(true)
        .open("crowfile")
        .unwrap();
    aliasfile.write_all(alias.as_bytes())
        .expect("Could not write to file");
}
