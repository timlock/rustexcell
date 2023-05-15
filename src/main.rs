use std::{env, fs};

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    let path = args.get(0).expect("File path missing");
    let file = fs::read_to_string(path);
}
