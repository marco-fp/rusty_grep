use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename).expect("File not found!");
    let mut contents = String::new();

    f.read_to_string(&mut contents)?;

    println!("With text : {}", contents);
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        Ok(
            Config {
                query: args[1].clone(),
                filename: args[2].clone()
            }
        )
    }
}

