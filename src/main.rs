// imports etc.
mod aliases;
mod managefiles;
#[macro_use]
extern crate clap;
use clap::{Arg, App};

fn main() {
    let crow = App::new("<>>>- Crow -<<<>")
       .author("Cameron Mills <cameron@dalesnail.surf>")
       .version(crate_version!())
       .about("Crow is a file manager for accessing and changing configuration files more conveniently")
       .arg(Arg::with_name("alias")
            .short("a")
            .long("alias")
            .value_name("ALIAS")
            .help("Set an alias for your config files. Provide the entire file path.")
            .takes_value(true))
       .arg(Arg::with_name("setalias")
            .short("s")
            .long("setalias")
            .value_name("'ALIAS FILEPATH'")
            .help("Set an alias for your config files. Provide the entire file path.")
            .takes_value(true))
       .arg(Arg::with_name("pull")
            .short("P")
            .long("pull")
            .value_name("PULL FILES INTO CROWFILES")
            .help("Takes the file listed for the named alias, and moves it to the nest")
            .takes_value(true))
            .get_matches();

    
    if let (Some(setalias), Some(alias)) = (crow.value_of("setalias"), crow.value_of("alias")) {
            aliases::definealias(alias.to_string(), setalias.to_string());
    }else if let Some(open) = crow.value_of("alias") { 
        aliases::openalias(open.to_string()); 
    }else if let Some(pull) = crow.value_of("pull") {
        managefiles::pull(pull.to_string());

    }

}
