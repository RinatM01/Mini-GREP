use std::{fs, error::Error};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let res = match search(&config.query, &contents) {
        Some(lines) => {
            for line in lines {
                println!("{line}")
            }
            Ok(())
        },
        None => Err("Nothing matches!!!")
    };
    Ok(res?)
}

pub fn search<'a>(query: &str, contents: &'a str) -> Option<Vec<&'a str>> {
    let mut res: Vec<&'a str> = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            res.push(line)
        }   
    }
    if res.len() == 0 {return None}
    Some(res)
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "rot";
        let contents = "\
Fruits:
apple, pear, carrot.
Find a vegetable.";
        
        assert_eq!(vec!["apple, pear, carrot."], search(query, contents).unwrap()); 
    }

    #[test]
    fn none_result() {
        let query = "rust";
        let contents = "\
Fruits:
apple, pear, carrot.
Find a vegetable.";
        assert!(search(query, contents).is_none());
    }
}