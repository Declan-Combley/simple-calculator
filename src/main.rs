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
            //let answer: f64 = calculate(numbers, operators);

            order_pairs(numbers, operators);

            // println!("Answer: {}", answer);
        }
    };
}

// TODO: make this function return the values and operators according to the order of operations
fn order_pairs(_numbers: Vec<f64>, operators: Vec<char>) {
    for (index, operator) in operators.iter().enumerate() {
        match operator {
            'x' | '/' | '%' | '^' => println!("{} needs to move", operators[index]),
            _ => println!("{}", operators[index]),
        }
    }
}

fn _calculate(mut numbers: Vec<f64>, mut operators: Vec<char>) -> f64 {
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
