use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let test = &args[1];
    println!("{}", test);
}
