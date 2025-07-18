// This file exposes the calculator functionality as a library
// so that it can be tested more easily

use std::fmt;

/// Represents the supported calculator operations
#[derive(Debug, PartialEq)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

/// Represents a calculation with two operands and an operation
#[derive(Debug, PartialEq)]
pub struct Calculation {
    pub left_operand: f64,
    pub right_operand: f64,
    pub operation: Operation,
}

/// Custom error type for calculator operations
#[derive(Debug, PartialEq)]
pub enum CalculatorError {
    InvalidInput(String),
    DivisionByZero,
    UnknownOperation(String),
}

impl fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalculatorError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            CalculatorError::DivisionByZero => write!(f, "Error: Division by zero is not allowed"),
            CalculatorError::UnknownOperation(op) => write!(f, "Unknown operation: {}", op),
        }
    }
}

/// Parse the input string into a Calculation struct
/// 
/// # Arguments
/// * `input` - A string slice containing the expression to parse
/// 
/// # Returns
/// * `Result<Calculation, CalculatorError>` - The parsed calculation or an error
pub fn parse_input(input: &str) -> Result<Calculation, CalculatorError> {
    // Trim whitespace and split the input by spaces
    let tokens: Vec<&str> = input.trim().split_whitespace().collect();
    
    // Check if we have exactly 3 tokens (left operand, operator, right operand)
    if tokens.len() != 3 {
        return Err(CalculatorError::InvalidInput(
            "Expression must be in the format 'number operator number'".to_string()
        ));
    }
    
    // Parse the left operand
    let left_operand = match tokens[0].parse::<f64>() {
        Ok(num) => num,
        Err(_) => return Err(CalculatorError::InvalidInput(
            format!("Could not parse '{}' as a number", tokens[0])
        )),
    };
    
    // Parse the operator
    let operation = match tokens[1] {
        "+" => Operation::Add,
        "-" => Operation::Subtract,
        "*" => Operation::Multiply,
        "/" => Operation::Divide,
        op => return Err(CalculatorError::UnknownOperation(op.to_string())),
    };
    
    // Parse the right operand
    let right_operand = match tokens[2].parse::<f64>() {
        Ok(num) => num,
        Err(_) => return Err(CalculatorError::InvalidInput(
            format!("Could not parse '{}' as a number", tokens[2])
        )),
    };
    
    // Return the parsed calculation
    Ok(Calculation {
        left_operand,
        right_operand,
        operation,
    })
}

/// Perform the calculation based on the provided Calculation struct
/// 
/// # Arguments
/// * `calculation` - A reference to a Calculation struct
/// 
/// # Returns
/// * `Result<f64, CalculatorError>` - The result of the calculation or an error
pub fn perform_calculation(calculation: &Calculation) -> Result<f64, CalculatorError> {
    let left = calculation.left_operand;
    let right = calculation.right_operand;
    
    match calculation.operation {
        Operation::Add => Ok(left + right),
        Operation::Subtract => Ok(left - right),
        Operation::Multiply => Ok(left * right),
        Operation::Divide => {
            // Check for division by zero
            if right == 0.0 {
                Err(CalculatorError::DivisionByZero)
            } else {
                Ok(left / right)
            }
        }
    }
}