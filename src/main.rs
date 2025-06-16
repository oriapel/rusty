use std::io::stdin;

/// This function check the result of the operation and print it
fn print_result(result: f64) -> () {
    match result {
        f64::INFINITY => println!("Result: Infinity"),
        f64::NEG_INFINITY => println!("Result: Negative Infinity"),
        _ if result.is_nan() => println!("Result: NaN"),
        _ => println!("Result: {result}"),
    }
}

/// This function reads a number from the user, and make sure that it is a valid number.
fn get_number_from_user() -> f64 {
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Failed to read from stdin :(");
    input.trim().parse().expect("Input is not a number!")
}

/// This function reads mathematic action from the user
fn get_action_from_user() -> String {
    let mut action = String::new();
    stdin()
        .read_line(&mut action)
        .expect("Failed to read from stdin :(");
    action.trim().to_string()
}

/// This function perform the desired calculation
fn do_math(first_num: f64, second_num: f64, action: &str) -> f64 {
    match action {
        "-" => return first_num - second_num,
        "+" => return first_num + second_num,
        "*" => return first_num * second_num,
        "/" => {
            if second_num == 0.0 {
                // This is not necessary as validate_result function will handle it,
                // but I kinda love the message printed so I left it here.
                println!("can't divide by 0 :(((((((");
                return f64::NAN;
            } else {
                return first_num / second_num;
            }
        }
        _ => {
            println!("What kind of dark magic are you dealing with -_-");
            return 0.0;
        }
    }
}

fn main() {
    println!("Welcome to the ~cyber~ calculator!");

    println!("Please enter your first number:");
    let first_number = get_number_from_user();

    println!("Please enter your second number:");
    let second_number = get_number_from_user();

    println!("Enter operation (+, -, *, /):");
    let operation = get_action_from_user();

    let result = do_math(first_number, second_number, &operation);
    print_result(result);
    println!("Thank you for using the ~cyber~ calculator!");
}
