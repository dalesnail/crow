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
            .value_name("FILE")
            .help("Set an alias for your config files. Provide the entire file path.")
            .takes_value(true))
       .get_matches();

    // Pulls the given argument, and converts it into a string, to feed to aliases.rs. Will need to
    // update this later to write that string to a plain text file.
    if let Some(alias) = crow.value_of("alias") {
        aliases::run(alias.to_string());
    }
}
