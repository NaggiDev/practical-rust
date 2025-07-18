// This file contains tests for the calculator functionality
use calculator::{parse_input, perform_calculation, Calculation, Operation, CalculatorError};

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for parse_input function
    #[test]
    fn test_parse_valid_addition() {
        let result = parse_input("5 + 3");
        assert!(result.is_ok());
        
        let calculation = result.unwrap();
        assert_eq!(calculation.left_operand, 5.0);
        assert_eq!(calculation.right_operand, 3.0);
        assert!(matches!(calculation.operation, Operation::Add));
    }

    #[test]
    fn test_parse_valid_subtraction() {
        let result = parse_input("10 - 7");
        assert!(result.is_ok());
        
        let calculation = result.unwrap();
        assert_eq!(calculation.left_operand, 10.0);
        assert_eq!(calculation.right_operand, 7.0);
        assert!(matches!(calculation.operation, Operation::Subtract));
    }

    #[test]
    fn test_parse_valid_multiplication() {
        let result = parse_input("4 * 6");
        assert!(result.is_ok());
        
        let calculation = result.unwrap();
        assert_eq!(calculation.left_operand, 4.0);
        assert_eq!(calculation.right_operand, 6.0);
        assert!(matches!(calculation.operation, Operation::Multiply));
    }

    #[test]
    fn test_parse_valid_division() {
        let result = parse_input("8 / 2");
        assert!(result.is_ok());
        
        let calculation = result.unwrap();
        assert_eq!(calculation.left_operand, 8.0);
        assert_eq!(calculation.right_operand, 2.0);
        assert!(matches!(calculation.operation, Operation::Divide));
    }

    #[test]
    fn test_parse_invalid_format() {
        let result = parse_input("5 + 3 + 2");
        assert!(result.is_err());
        
        match result {
            Err(CalculatorError::InvalidInput(_)) => assert!(true),
            _ => assert!(false, "Expected InvalidInput error"),
        }
    }

    #[test]
    fn test_parse_invalid_number() {
        let result = parse_input("abc + 3");
        assert!(result.is_err());
        
        match result {
            Err(CalculatorError::InvalidInput(_)) => assert!(true),
            _ => assert!(false, "Expected InvalidInput error"),
        }
    }

    #[test]
    fn test_parse_invalid_operator() {
        let result = parse_input("5 $ 3");
        assert!(result.is_err());
        
        match result {
            Err(CalculatorError::UnknownOperation(op)) => assert_eq!(op, "$"),
            _ => assert!(false, "Expected UnknownOperation error"),
        }
    }

    // Tests for perform_calculation function
    #[test]
    fn test_perform_addition() {
        let calculation = Calculation {
            left_operand: 5.0,
            right_operand: 3.0,
            operation: Operation::Add,
        };
        
        let result = perform_calculation(&calculation);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 8.0);
    }

    #[test]
    fn test_perform_subtraction() {
        let calculation = Calculation {
            left_operand: 10.0,
            right_operand: 7.0,
            operation: Operation::Subtract,
        };
        
        let result = perform_calculation(&calculation);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 3.0);
    }

    #[test]
    fn test_perform_multiplication() {
        let calculation = Calculation {
            left_operand: 4.0,
            right_operand: 6.0,
            operation: Operation::Multiply,
        };
        
        let result = perform_calculation(&calculation);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 24.0);
    }

    #[test]
    fn test_perform_division() {
        let calculation = Calculation {
            left_operand: 8.0,
            right_operand: 2.0,
            operation: Operation::Divide,
        };
        
        let result = perform_calculation(&calculation);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 4.0);
    }

    #[test]
    fn test_perform_division_by_zero() {
        let calculation = Calculation {
            left_operand: 5.0,
            right_operand: 0.0,
            operation: Operation::Divide,
        };
        
        let result = perform_calculation(&calculation);
        assert!(result.is_err());
        
        match result {
            Err(CalculatorError::DivisionByZero) => assert!(true),
            _ => assert!(false, "Expected DivisionByZero error"),
        }
    }

    // Integration tests that combine parsing and calculation
    #[test]
    fn test_integration_valid_expression() {
        let input = "5 + 3";
        let calculation = parse_input(input).unwrap();
        let result = perform_calculation(&calculation).unwrap();
        assert_eq!(result, 8.0);
    }

    #[test]
    fn test_integration_floating_point() {
        let input = "3.5 * 2.0";
        let calculation = parse_input(input).unwrap();
        let result = perform_calculation(&calculation).unwrap();
        assert_eq!(result, 7.0);
    }

    #[test]
    fn test_integration_division_by_zero() {
        let input = "5 / 0";
        let calculation = parse_input(input).unwrap();
        let result = perform_calculation(&calculation);
        assert!(result.is_err());
    }
}