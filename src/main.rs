use std::env;  

fn is_even(x: usize) -> bool {
    if x % 2 == 0 { return true }
    false
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // incorrect args given 
        1 | 2 | 3 => panic!("Expected 3 or more arguments"),
        // all other cases
        _ => {
            let mut numbers: Vec<f64> = Vec::new(); 
            let mut operators: Vec<char> = Vec::new();

            let mut n: usize = 0;
            
            // pushes all of the operators and numbers into their own vectors
            // for now this is done based off of the order of input, in that every second 
            // argument is going to be an operator, but basic algebra would not be particularly co-operative with that idea
            for arg in env::args() {
                if is_even(n) == false { numbers.push(arg.parse::<f64>().unwrap()) } 
                else { operators.push(arg.chars().next().unwrap()); }
                n = n + 1;
            }
            // for some reason `t` is always the first index
            operators.remove(0);

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
        n = n + 1;
    } 
}

fn calculate_pair(number_1: f64, number_2: f64, operator: char) -> f64 {
    match operator {
        '+' => return number_1 + number_2,
        '-' => return number_1 - number_2,
        'x' => return number_1 * number_2,
        '/' => return number_1 / number_2,
        '%' => return number_1 % number_2,
        '^' => return f64::powf(number_1, number_2),
        _ => panic!("Invalid operator"),
    }
}

// fn order_operator(operator: Vec<String>) -> Vec<String> { operator }