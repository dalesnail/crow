use std::fs::OpenOptions;
use std::io::Write;

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
