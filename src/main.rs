//! This is a simple program that get numbers and action from the user,
//! and perform the wanted calculation accordingly
use std::io::stdin;
use rusty::calculator::do_math;

/// Reads a f64 number from the user input
fn get_f64_from_user() -> Result<f64, Box<dyn std::error::Error>> {
    let mut input = String::new();

    stdin().read_line(&mut input)?;

    let number = input.trim().parse::<f64>()?;

    Ok(number)
}

/// Reads a string from the user input
fn get_text_from_user() -> Result<String, Box<dyn std::error::Error>> {
    let mut input = String::new();

    stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}

fn main() {
    println!("Welcome to the ~cyber~ calculator!");

    println!("Please enter your first number:");
    let first_number = match get_f64_from_user() {
                Ok(num) => num,
                Err(e) => {
                    println!("Error: {e}");
                    return;
                }
            };

    println!("Please enter your second number:");

    let second_number = match get_f64_from_user() {
                Ok(num) => num,
                Err(e) => {
                    println!("Error: {e}");
                    return;
                }
            };

    println!("Enter operation (+, -, *, /):");
    let operation = match get_text_from_user() {
                        Ok(text) => text,
                        Err(e) => {
                            println!("Error reading text: {e}");
                            return;
                        }
                    };

    match do_math(first_number, second_number, &operation) {
        Ok(ans) => println!("answer is {ans}"),
        Err(e) => println!("Error! {e}"),
    }

    println!("Thank you for using the ~cyber~ calculator!");
}
