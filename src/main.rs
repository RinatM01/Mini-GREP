
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];
    println!("Query: {}", query);
    println!("Path: {}", file_path);

    let file = fs::read_to_string(file_path).expect("There should be existing file!");

    println!("Contents of file: \n{file}");
    // dbg!(args);
    //cargo run -- lol src/text.txt
}
