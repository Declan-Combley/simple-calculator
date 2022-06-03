use std::env;

fn is_even(number: usize) -> bool {
    if number % 2 == 0 {
        return true;
    }
    false
}

fn main() {
    let args = env::args();

    match args.len() {
        1 | 2 | 3 => panic!("Expected 3 or more arguments"),
        _ => {
            let mut numbers: Vec<f64> = Vec::new();
            let mut operators: Vec<char> = Vec::new();

            for (n, arg) in env::args().enumerate() {
                if !is_even(n) {
                    numbers.push(arg.parse::<f64>().unwrap())
                } else {
                    operators.push(arg.chars().next().unwrap());
                }
            }
            operators.remove(0); // for some reason `t` is always the first index

            let answer: f64 = calculate(numbers, operators);
            
            println!("{}", answer);
        }
    };
}

fn calculate(mut numbers: Vec<f64>, mut operators: Vec<char>) -> f64 {
    let len: usize = numbers.len();
    let mut index: usize = 0;

    let mut values: Vec<f64> = vec![calculate_pair(
        numbers[index],
        operators[index],
        numbers[index + 1],
    )];

    numbers.remove(1);
    numbers.remove(0);
    operators.remove(0);

    while index + 1 < len - 1 {
        values.push(calculate_pair(values[0], operators[0], numbers[0]));
        numbers.remove(0);
        operators.remove(0);
        values.remove(0);

        index += 1;
    }
    values[0]
}

fn calculate_pair(number_1: f64, operator: char, number_2: f64) -> f64 {
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
