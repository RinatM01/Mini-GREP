use std::{fs, error::Error};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string(config.file_path)?;

    println!("Contents of file: \n{file}");
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!!!");
        }
        return Ok(Config {
            query: args[1].clone(), 
            file_path: args[2].clone()
        })
    }
}