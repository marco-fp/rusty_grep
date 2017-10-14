use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename).expect("File not found!");
    let mut contents = String::new();

    f.read_to_string(&mut contents)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

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

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn one_result() {
        let query = "duct";
        let contents= "\
            Rust: safe, fast, productive.
            Pick three.
        ";

        assert_eq!(
            vec!["Rust: safe, fast, productive."], 
            search(query, contents)
        );
    }
}
