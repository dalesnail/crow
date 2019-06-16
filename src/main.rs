// imports etc.
mod aliases;
#[macro_use]
extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {
    let crow = App::new("<>>>- Crow -<<<>")
       .author("Cameron Mills <cameron@dalesnail.surf>")
       .version(crate_version!())
       .about("Crow is a file manager for accessing and changing configuration files more conveniently")
       .after_help("Copyright (C) 2019  Cameron Mills
This program is free software: you can redistribute it and/or modify it under the terms
of the GNU General Public License as published by the Free Software Foundation, either
version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program.
If not, see <https://www.gnu.org/licenses/>.")
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
            .get_matches();

    
    if let (Some(setalias), Some(alias)) = (crow.value_of("setalias"), crow.value_of("alias")) {
            aliases::definealias(alias.to_string(), setalias.to_string());
    }else if let Some(open) = crow.value_of("alias") { 
        aliases::openalias(open.to_string()); 
    }

}
