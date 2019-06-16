use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::fs::OpenOptions;
use std::process::Command;
use std::path::PathBuf;
extern crate directories;
use directories::ProjectDirs;
use std::string::String;
//use std::io::Read;


// I have tried a lot of things to get this to locate an existing alias, and change it in place,
// but I cannot get that to work. I will need to learn more about this and try and get it working
// at some point, but for now, all duplicates will need to be removed by hand.
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
