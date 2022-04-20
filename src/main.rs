use std::env;  

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
               if n % 2 != 0 { numbers.push(arg.parse().unwrap()) } else { operators.push(arg) }
               n = n + 1;
            }
            operators.remove(0);

            for operator in operators { println!("{:?}", operator) };
            for number in numbers { println!("{:?}", number) };
            println!("done...");
        }
    };
}