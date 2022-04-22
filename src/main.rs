use std::env;  

fn is_even(number: usize) -> bool {
    if number % 2 == 0 { return true }
    false
}

fn main() {
    let args = env::args();

    match args.len() {
        // incorrect args given 
        1 | 2 | 3 => panic!("Expected 3 or more arguments"),
        // all other cases
        _ => {
            let mut numbers: Vec<f64> = Vec::new(); 
            let mut operators: Vec<char> = Vec::new();

            // pushes all of the operators and numbers into their own vectors, based off of the index being odd
            for (n, arg) in env::args().enumerate() {
                if !is_even(n) { numbers.push(arg.parse::<f64>().unwrap()) } 
                else { operators.push(arg.chars().next().unwrap()); }
            }
            operators.remove(0); // for some reason `t` is always the first index

            order_pairs(numbers, operators);
        }
    };
}

// -> (n1, n2, op)
fn order_pairs(numbers: Vec<f64>, operators: Vec<char>) {
    let mut n: usize = 0;
    let len: usize = numbers.len();
    
    while n < len - 1 {
        println!("{} {} {}", numbers[n], operators[n], numbers[n + 1]);
        n += 1;
    } 
}

fn calculate_pair(number_1: f64, number_2: f64, operator: char) -> f64 {
    match operator {
        '+' => number_1 + number_2,
        '-' => number_1 - number_2,
        'x' => number_1 * number_2,
        '/' => number_1 / number_2,
        '%' => number_1 % number_2,
        '^' => f64::powf(number_1, number_2),
        _ => panic!("Invalid operator"),
    }
}

// fn order_operator(operator: Vec<String>) -> Vec<String> { operator }