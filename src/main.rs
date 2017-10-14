extern crate rusty_grep;

use std::env;
use std::process;
use rusty_grep::Config;

fn main() {
    
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else ( |error| {
        println!("Problem parsing arguments: {}", error);
        process::exit(1);
    });
    
    println!("Searching for: {}", config.query);
    println!("In file: {}", config.filename);
    if let Err(e) = rusty_grep::run(config) {
        println!("Aplication error: {}", e);
        process::exit(1);
    };
}



