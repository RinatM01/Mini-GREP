use std::{env, error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let res = match search(&config.query, config.ignore_case, &contents) {
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

pub fn search<'a>(query: &str, ignore_case: bool, contents: &'a str) -> Option<Vec<&'a str>> {
    let mut res: Vec<&'a str> = vec![];
    for line in contents.lines() {
        if ignore_case {
            let query = query.to_lowercase();
            if line.to_lowercase().contains(&query) {
                res.push(line)
            }   
        } else {
            if line.contains(&query) {
                res.push(line)
            }   
        }
        
    }
    if res.len() == 0 {return None}
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
        
        assert_eq!(vec!["apple, pear, carrot."], search(query, false, contents).unwrap()); 
    }

    #[test]
    fn one_result_case_insensetive() {
        let query = "Its";
        let contents = "\
FruitS:
apple, pear, carRot.
Find a vegetable.
its type is not a fruite";
        
        assert_eq!(vec!["FruitS:","its type is not a fruite"], search(query, true, contents).unwrap()); 
    }      

    #[test]
    fn none_result() {
        let query = "rust";
        let contents = "\
Fruits:
apple, pear, carrot.
Find a vegetable.";
        assert!(search(query, true, contents).is_none());
    }
}