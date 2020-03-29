use std::env;
use std::io::{stdin};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut map_file = String::new();
    let mut input= String::new();
    if args.get(1).is_some() {
        map_file = args[1].clone();
    }
    println!("{}", map_file);
    stdin().read_line(&mut input); // wait for next enter key press
}