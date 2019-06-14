use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;
use std::path::PathBuf;
use std::path::Path;
extern crate directories;
use directories::{BaseDirs, UserDirs, ProjectDirs};

pub fn definealias(alias: String, filepath: String) {
    let alias = "-<<<>  ".to_owned() + &alias + ": " + &filepath + "\n";
    if let Some(home) = ProjectDirs::from("com", "gnucrow", "crow") {
        let mut path = PathBuf::from(home.config_dir());
        path.push("crowfile");
        let mut aliasfile = OpenOptions::new()
            .read(true)
            .append(true)
            .open(&path)
            .unwrap();
        aliasfile.write_all(alias.as_bytes())
            .expect("Could not write to file");
    }
}

pub fn openalias(alias: String) {
    let srchterm = "-<<<>  ".to_owned() + &alias + ": ";
    if let Some(home) = ProjectDirs::from("com", "gnucrow", "crow") {
        let mut path = PathBuf::from(home.config_dir());
        path.push("crowfile");
        let aliasfile = BufReader::new(File::open(&path).unwrap());
            for line in aliasfile.lines() {
                match line { 
                    Ok(line) => if line.starts_with(&srchterm) {
                        let aliaspath: Vec<_> = line.split(": ").collect();
                        let file = &aliaspath[1]; 
                        Command::new("nvim")
                            .arg(file)
                            .status()
                            .expect("Something went wrong.");
                        println!("Path: {:?}", &path);
                    },
                Err(e) => panic!("Error reading file: {}", e)
                }
        }
    }
}
