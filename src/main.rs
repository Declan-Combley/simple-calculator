use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // incorrect args given 
        1 | 2 | 3 => panic!("Expected 3 or more arguments"),
        // all other cases
        _ => get_numbers(args.len()) 
    }
}

fn get_numbers(arg_len: usize) {
    let mut input: Vec<String> = Vec::new();

    for argument in env::args() {
        println!("{}", argument);
        input.push(argument);
    }
}