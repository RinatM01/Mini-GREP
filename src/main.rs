
use std::{env, process};
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Error parsing input: {err}");
        process::exit(1);
    });
    println!("Query: {}", config.query);
    println!("Path: {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("Error caught in main: {e}");
        process::exit(1);
    }
    // dbg!(args);
    //cargo run -- lol src/text.txt
}
