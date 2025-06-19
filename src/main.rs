mod calculator;

use std::io::stdin;

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

fn main() {
    println!("Welcome to the ~cyber~ calculator!");

    println!("Please enter your first number:");
    let first_number = get_number_from_user();

    println!("Please enter your second number:");
    let second_number = get_number_from_user();

    println!("Enter operation (+, -, *, /):");
    let operation = get_action_from_user();

    let mut calculator_answer = calculator::CalculatorAnswer::new();

    calculator_answer.do_math(first_number, second_number, &operation);
    println!("{}", calculator_answer);
    println!("Thank you for using the ~cyber~ calculator!");
}
