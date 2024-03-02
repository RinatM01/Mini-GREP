use std::{env, error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let res = if config.ignore_case {
        search_insensetive(&config.query, &contents)
    } else {
        search_sensetive(&config.query, &contents)
    };
    let res = match res {
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

pub fn search_insensetive<'a>(query: &str, contents: &'a str) -> Option<Vec<&'a str>> {
    let res: Vec<&str> = contents.lines().filter(|line| line.to_lowercase().contains(&query)).collect();
    if res.len() == 0 {return None;}
    Some(res)
}

pub fn search_sensetive<'a>(query: &str, contents: &'a str) -> Option<Vec<&'a str>> {
    let res: Vec<&str> = contents.lines().filter(|line| line.contains(&query)).collect();
    if res.len() == 0 {return None;}
    Some(res)
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!!!");
        }
        return Ok(Config {
            query: args[1].clone(), 
            file_path: args[2].clone(),
            ignore_case: env::var("IGNORE_CASE").is_ok()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result_case_sensetive() {
        let query = "rot";
        let contents = "\
Fruits:
apple, pear, carrot.
Find a vegetable.";
        
        assert_eq!(vec!["apple, pear, carrot."], search_sensetive(query, contents).unwrap()); 
    }

    #[test]
    fn one_result_case_insensetive() {
        let query = "its";
        let contents = "\
FruitS:
apple, pear, carRot.
Find a vegetable.
its type is not a fruite";
        
        assert_eq!(vec!["FruitS:","its type is not a fruite"], search_insensetive(query, contents).unwrap()); 
    }      

    #[test]
    fn none_result() {
        let query = "rust";
        let contents = "\
Fruits:
apple, pear, carrot.
Find a vegetable.";
        assert!(search_sensetive(query, contents).is_none());
    }
}