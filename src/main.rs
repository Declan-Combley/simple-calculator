use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // incorrect args given 
        1 | 2 | 3 => panic!("Expected 3 or more arguments"),
        // all other cases
        _ => {
            let input: Vec<String> = get_numbers();
            calculate(input);
            println!("done...")
        }
    };
}

fn is_odd(number: usize) -> bool {
    if number % 2 == 0 { return false; }
    true
}

fn get_numbers() -> Vec<String> {
    let mut input: Vec<String> = Vec::new();

    for argument in env::args() {
        input.push(argument);
    }
    input
}

fn calculate(input: Vec<String>) {
    let mut numbers: Vec<f64> = Vec::new();
    let mut operators: Vec<char> = Vec::new();
    let mut n = 0;

    for argument in input {
        if is_odd(n) == true { 
            let number = argument.parse().unwrap();
            numbers.push(number);
        } else {
            let operator = argument.chars().next().unwrap();
            operators.push(operator);
        }
        n = n + 1;
    }
    operators.remove(0);
    
    println!("Operators: ");
    for operator in operators {
        println!("{:?}", operator);
    }

    println!("Numbers: ");
    for number in numbers {
        println!("{:?}", number);
    }
}