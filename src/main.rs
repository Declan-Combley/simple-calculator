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
        // incorrect args given
        1 | 2 | 3 => panic!("Expected 3 or more arguments"),
        // all other cases
        _ => {
            let mut numbers: Vec<f64> = Vec::new();
            let mut operators: Vec<char> = Vec::new();

            // pushes all of the operators and numbers into their own vectors
            for (n, arg) in env::args().enumerate() {
                if !is_even(n) {
                    numbers.push(arg.parse::<f64>().unwrap())
                } else {
                    operators.push(arg.chars().next().unwrap());
                }
            }
            operators.remove(0); // for some reason `t` is always the first index

            //order_pairs(numbers, operators);
            calculate(numbers, operators)
        }
    };
}

// TODO: Make this work
fn _order_pairs(numbers: Vec<f64>, operators: Vec<char>) {
    let len: usize = numbers.len();
    let mut index: usize = 0;

    while index < len - 1 {
        println!(
            "{} {} {}",
            numbers[index],
            operators[index],
            numbers[index + 1]
        );
        index += 1;
    }
}

fn calculate(mut numbers: Vec<f64>, mut operators: Vec<char>) {
    let len: usize = numbers.len();
    let mut index: usize = 0;

    let mut values: Vec<f64> = Vec::new();

    // calculate the first sum and remove the operator and first two numbers
    values.push(calculate_pair(
        numbers[index],
        operators[index],
        numbers[index + 1],
    ));

    numbers.remove(0);
    operators.remove(0);

    while index + 1 < len - 1 {
        values.push(calculate_pair(values[0], operators[0], numbers[0]));
        numbers.remove(0);
        operators.remove(0);
        values.remove(0);

        index += 1;
    }

    for ans in values {
        println!("{}", { ans });
    }
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
