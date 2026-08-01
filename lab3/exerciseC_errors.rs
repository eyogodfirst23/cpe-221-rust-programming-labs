use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
enum AppError {
    Parse(ParseIntError),
    OutOfRange { value: i32, min: i32, max: i32 },
    EmptyInput,
    DivisibleByZero,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Parse(e) => write!(f, "Parse error: {}", e),
            AppError::OutOfRange { value, min, max } => {
                write!(f, "{} is not in [{}, {}]", value, min, max)
            }
            AppError::EmptyInput => write!(f, "Input was empty"),
            AppError::DivisibleByZero => write!(f, "Cannot divide by zero"),
        }
    }
}

impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::Parse(e)
    }
}

fn parse_and_validate(s: &str, min: i32, max: i32) -> Result<i32, AppError> {
    if s.is_empty() {
        return Err(AppError::EmptyInput);
    }
    let n: i32 = s.trim().parse()?;
    if n < min || n > max {
        return Err(AppError::OutOfRange { value: n, min, max });
    }
    Ok(n)
}

fn safe_div(a: i32, b: i32) -> Result<i32, AppError> {
    if b == 0 {
        return Err(AppError::DivisibleByZero);
    }
    Ok(a / b)
}

fn main() {
    let test_cases = vec!["42", " 101 ", "abc", "", "-5"];
    for case in test_cases {
        match parse_and_validate(case, 0, 100) {
            Ok(n) => println!("Valid: {}", n),
            Err(e) => println!("Error for {:?}: {}", case, e),
        }
    }

    match safe_div(10, 2) {
        Ok(n) => println!("10 / 2 = {}", n),
        Err(e) => println!("Error: {}", e),
    }
    match safe_div(10, 0) {
        Ok(n) => println!("10 / 0 = {}", n),
        Err(e) => println!("Error: {}", e),
    }
              }
