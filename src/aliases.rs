use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::fs::OpenOptions;
use std::process::Command;
use std::path::PathBuf;
extern crate directories;
use directories::{ProjectDirs, UserDirs};
use std::string::String;
use std::fs;

// I have tried a lot of things to get this to locate an existing alias, and change it in place,
// but I cannot get that to work. I will need to learn more about this and try and get it working
// at some point, but for now, all duplicates will need to be removed by hand.
pub fn definealias(alias: String, filepath: String) {
    let alias = "-<<<>  ".to_owned() + &alias + ": " + &filepath + "\n";
    if let Some(home) = ProjectDirs::from("com", "gnucrow", "crow") {
        let mut path = PathBuf::from(home.config_dir());
        path.push("crowfile");
        if path.exists() {
            let mut aliasfile = OpenOptions::new()
                .read(true)
                .append(true)
                .open(&path)
                .unwrap();
            aliasfile.write_all(alias.as_bytes())
                .expect("Could not write to file");
        } else {
            fs::create_dir(&home.config_dir()).expect("Cannot create folder");
            let _createcrowfile = File::create(&path);
            let mut newfile = OpenOptions::new()
                .read(true)
                .append(true)
                .open(&path)
                .unwrap();
            let init = format!("Editor: vim\n# The above line is for declaring your editor, line should stay at the very top\n-<<<>  crowfile: {}\n{}", &path.display(), &alias);
            newfile.write_all(init.as_bytes())
                .expect("Could not write to file");
            println!("Testing"); 
        }
    }
} 
pub fn openalias(alias: String) {
    let srchterm = "-<<<>  ".to_owned() + &alias + ": ";
    if let Some(home) = ProjectDirs::from("com", "gnucrow", "crow") {
        let mut path = PathBuf::from(home.config_dir());
        path.push("crowfile");
        let aliasfile = BufReader::new(File::open(&path).unwrap());
        let mut lines = aliasfile.lines();
        let editor_line = &lines.next().unwrap().expect("Error");
        let editor_vec: Vec<_> = editor_line.split(": ").collect();
        let editor = &editor_vec[1];
            for line in lines {
                match line { 
                    Ok(line) => if line.starts_with(&srchterm) {
                        let aliaspath: Vec<_> = line.split(": ").collect();
                        let file = &aliaspath[1]; 
                        Command::new(&editor)
                            .arg(file)
                            .status()
                            .expect("Something went wrong.");
                        println!("Path: {:?}", &editor);
                    },
            Err(e) => panic!("Error reading file: {}", e)
                }
            }
        }
    }

pub fn definegroup(group: String) {
    let groupalias = "-<<<+>  ".to_owned() + &group + ": ";
    if let Some(home) = UserDirs::new() {
        let mut path = PathBuf::from(home.home_dir());
        path.push("nest");
        let groupdir = format!("{}/{}", &path.display(), &group);
        let writega = format!("{}{}\n", &groupalias, &groupdir);
        if let Some(crowfile) = ProjectDirs::from("com", "gnucrow", "crow") {
            let mut cpath = PathBuf::from(crowfile.config_dir());
            cpath.push("crowfile");
            if cpath.exists() {
                let mut aliasfile = OpenOptions::new()
                    .read(true)
                    .append(true)
                    .open(&cpath)
                    .unwrap();
                aliasfile.write_all(writega.as_bytes())
                    .expect("Could not write to file");
                fs::create_dir_all(&groupdir).expect("could not create folder!");
            } else {
                fs::create_dir(&crowfile.config_dir()).expect("Cannot create folder");
                let _createcrowfile = File::create(&path);
                let mut newfile = OpenOptions::new()
                    .read(true)
                    .append(true)
                    .open(&cpath)
                    .unwrap();
                newfile.write_all(writega.as_bytes())
                    .expect("Could not write to file");
                println!("Testing"); 
                fs::create_dir_all(&groupdir).expect("could not create folder!");
            }
        }
    }
}
