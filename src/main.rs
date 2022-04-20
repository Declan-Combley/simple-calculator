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
            let mut operators: Vec<String> = Vec::new();

            let mut n: usize = 0;

            for arg in env::args() {
               if is_even(n) == false { numbers.push(arg.parse().unwrap()) } 
               else { operators.push(arg) }
               n = n + 1;
            }
            operators.remove(0);
        }
    };
}

fn order_operator(operator: Vec<String>) -> Vec<String> {
    operator
}