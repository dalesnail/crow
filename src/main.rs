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
       .arg(Arg::with_name("setgroup")
            .short("G")
            .long("gset")
            .value_name("GROUP ALIAS")
            .help("Establishes a new group alias an creates the group dir in your nest")
            .takes_value(true))
       .arg(Arg::with_name("group")
            .short("g")
            .long("group")
            .value_name("GROUP ALIAS")
            .help("Argument for group alias, will cd into groups directory with no other arguments")
            .takes_value(true))
       .arg(Arg::with_name("init")
            .short("i")
            .long("init")
            .help("Creates reference files and directories for crow"))
            .get_matches();


    // Functions used for arguments 

    if let (Some(setalias), Some(alias)) = (crow.value_of("setalias"), crow.value_of("alias")) {
            aliases::definealias(alias.to_string(), setalias.to_string());
    }else if let (Some(alias), Some(group)) = (crow.value_of("alias"), crow.value_of("group")) {
            managefiles::groupull(alias.to_string(), group.to_string());
    }else if let Some(open) = crow.value_of("alias") { 
        aliases::openalias(open.to_string()); 
    }else if let Some(pull) = crow.value_of("pull") {
        managefiles::pull(pull.to_string());
    }else if let Some(gset) = crow.value_of("setgroup") {
        aliases::definegroup(gset.to_string());
    }else if crow.is_present("init") {
        aliases::init();

    }

}
