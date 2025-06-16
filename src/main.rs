use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

/// The max value of the secret number.
const MAX_VALUE: u32 = 100;

/// The min value of the secret number.
const MIN_VALUE: u32 = 1;

/// This function gets a number from the user.
fn get_number_from_user() -> u32 {
    let mut user_input = String::new();
    stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    user_input
        .trim()
        .parse()
        .expect("Please input a valid number")
}

/// Validates if the user's guess is within the valid range.
fn is_guess_valid(user_guess: u32) -> bool {
    return (MIN_VALUE..=MAX_VALUE).contains(&user_guess);
}

/// Validates the user's guess against the secret number, and print the result
fn check_guess(user_guess: u32, secret_number: u32) -> bool {
    match user_guess.cmp(&secret_number) {
        Ordering::Less => {
            println!("Number is too small!");
            return false;
        }
        Ordering::Greater => {
            println!("Number is too big!");
            return false;
        }
        Ordering::Equal => {
            println!("Correct!");
            return true;
        }
    }
}

fn main() {
    println!("Welcone to guess the number game!");

    // Generates a random secret number between 1 and 100.
    let secret_number: u32 = rand::thread_rng().gen_range(MIN_VALUE..=MAX_VALUE);

    println!("Guess a number! ({MIN_VALUE}-{MAX_VALUE})");

    loop {
        let user_guess = get_number_from_user();

        if !is_guess_valid(user_guess) {
            println!("Please input a valid number between {MIN_VALUE} and {MAX_VALUE}.");
            continue;
        }

        // If the function returns true, user has won
        if check_guess(user_guess, secret_number) {
            println!("Well done! Game over.");
            break;
        }
        println!("Try again!");
    }
}
