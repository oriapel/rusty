//! This is the implementation of calculator, that can get two numbers 
//! and calculation action, and perform the wanted calculation

use thiserror::Error;

/// Enum representing the possible errors that can occur during calculations
#[derive(Debug, Error, PartialEq)]
pub enum CalculatorError {
    #[error("Overflow occurred!")]
    Overflow,
    #[error("Underflow occurred!")]
    Underflow,
    #[error("Nan!")]
    Nan,
    #[error("Invalid operation!")]
    InvalidOperation,
    #[error("Division by 0!")]
    DivisionByZero,
}

/// This function perform the desired calculation, and check for erros along the way
pub fn do_math(first_num: f64, second_num: f64, action: &str) -> Result<f64, CalculatorError> {
    let ans = match action {
        "-" => first_num - second_num,
        "+" => first_num + second_num,
        "*" => first_num * second_num,
        "/" => {
            if second_num == 0.0 {
                return Err(CalculatorError::DivisionByZero);
            }
            first_num / second_num
        }
        _ => {
            println!("What kind of dark magic are you dealing with -_-");
            return Err(CalculatorError::InvalidOperation);
        }
    };

    match ans {
        f64::INFINITY => return Err(CalculatorError::Overflow),
        f64::NEG_INFINITY => return Err(CalculatorError::Underflow),
        _ if ans.is_nan() => return Err(CalculatorError::Nan),
        _ => return Ok(ans),
    }
}
