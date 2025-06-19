use std::fmt::Display;

/// Enum representing the possible errors that can occur during calculations
enum CalculatorError {
    Overflow,
    Underflow,
    Nan,
    InvalidOperation,
    DivisionByZero,
    AllOk,
}

/// Struct representing calculation answers.
///
/// This struct contains the result of the calculation and the status of the calculation.
///
/// # Examples
///
/// ```rust
/// use rusty::calculator::CalculatorAnswer;
///
/// let mut calculator_answer = CalculatorAnswer::new();
/// calculator_answer.do_math(5.0, 0.0, "/");
/// assert_eq!(format!("{}", calculator_answer), "Attempted to divide by zero");
/// ```
pub struct CalculatorAnswer {
    pub result: f64,
    status: CalculatorError,
}

impl Display for CalculatorAnswer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.status {
            CalculatorError::Overflow => write!(f, "Overflow occurred during calculation"),
            CalculatorError::Underflow => write!(f, "Underflow occurred during calculation"),
            CalculatorError::InvalidOperation => write!(f, "Invalid operation"),
            CalculatorError::DivisionByZero => write!(f, "Attempted to divide by zero"),
            CalculatorError::Nan => write!(f, "Result is NaN (Not a Number)"),
            CalculatorError::AllOk => write!(f, "Answer: {}", self.result),
        }
    }
}

impl CalculatorAnswer {
    /// Constructor for the calculator struct
    /// Initializes a new calculator with default values
    pub fn new() -> Self {
        Self {
            result: 0.0,
            status: CalculatorError::AllOk,
        }
    }

    fn check_calculation_errors(&mut self) {
        match self.result {
            f64::INFINITY => self.status = CalculatorError::Overflow,
            f64::NEG_INFINITY => self.status = CalculatorError::Underflow,
            _ if self.result.is_nan() => self.status = CalculatorError::Nan,
            _ => return, // Keep the current status if no error
        }
    }

    /// This function perform the desired calculation
    pub fn do_math(&mut self, first_num: f64, second_num: f64, action: &str) {
        self.status = CalculatorError::AllOk; // Reset status before calculation
        match action {
            "-" => self.result = first_num - second_num,
            "+" => self.result = first_num + second_num,
            "*" => self.result = first_num * second_num,
            "/" => {
                if second_num == 0.0 {
                    self.status = CalculatorError::DivisionByZero;
                } else {
                    self.result = first_num / second_num;
                }
            }
            _ => {
                println!("What kind of dark magic are you dealing with -_-");
                self.status = CalculatorError::InvalidOperation;
            }
        }

        self.check_calculation_errors();
    }
}
