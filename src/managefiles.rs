use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
extern crate directories;
use directories::{UserDirs, ProjectDirs};
use std::fs::File;
extern crate regex;
use regex::Regex;


pub fn pull(alias: String) {
    let srchterm = "-<<<>  ".to_owned() + &alias + ": ";
    if let Some(home) = UserDirs::new() {
        let mut path = PathBuf::from(home.home_dir());
        path.push("nest");
        let delimiter = format!("-<<<>  {}: {}", &alias, &home.home_dir().display());
        if let Some(crowfile) = ProjectDirs::from("come", "gnucrow", "crow") {
            let mut cfile = PathBuf::from(crowfile.config_dir());
            cfile.push("crowfile");
            let aliasfile = BufReader::new(File::open(&cfile).unwrap());
            for line in aliasfile.lines() {
                match line {
                    Ok(line) => if line.starts_with(&srchterm) {
                        let aliaspath: Vec<_> = line.split(&delimiter).skip(1).collect();
                        let newdir = format!("{}/{}{}", &path.display(), &alias, aliaspath[0]);
                        let chop = Regex::new(r"/(?:[^/]+)$").unwrap();
                        let choppedir = chop.replace(&newdir, "");
                        let finaldir = format!("{}", choppedir);
                        println!("{}", choppedir);
                        fs::create_dir_all(&finaldir).expect("Cannot Create Folder!");
                    },
            Err(e) => panic!("Error reading file: {}", e)
                }
            }
        }
    }
}
