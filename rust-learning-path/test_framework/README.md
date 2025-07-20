# Rust Learning Path Test Framework

This comprehensive test framework validates all code examples in the Rust Learning Path project. It ensures that all examples work correctly and provides clear feedback when issues are found.

## Features

- **Comprehensive Coverage**: Tests all code examples from Basic to Expert levels
- **Clear Error Messages**: Detailed failure messages help identify and fix issues
- **Automated Validation**: Continuous testing ensures examples remain working
- **Flexible Execution**: Run all tests, specific levels, or individual concepts
- **Performance Tracking**: Measures test execution time
- **Detailed Reporting**: Generates comprehensive test reports

## Quick Start

### Prerequisites

- Rust 1.70+ installed
- Python 3.7+ (for automation scripts)

### Running Tests

```bash
# Build and run all tests
cargo run

# Run tests for a specific level
cargo run -- --level basic

# Run tests for a specific concept
cargo run -- --concept ownership

# Validate all examples
cargo run -- --validate

# Show test statistics
cargo run -- --stats
```

### Using the Python Automation Script

```bash
# Run full test suite with report
python run_tests.py

# Quick validation only
python run_tests.py --quick

# Build framework only
python run_tests.py --build-only
```

## Test Organization

### Test Levels

- **Basic**: Variables, data types, control flow, functions, basic error handling
- **Intermediate**: Ownership, structs, enums, traits, generics, collections
- **Advanced**: Concurrency, unsafe Rust, advanced traits, macros, error patterns
- **Expert**: Async programming, memory management, performance, compiler internals

### Test Concepts

Each concept area has dedicated test suites:

- `variables` - Variable declaration and mutability
- `data_types` - Primitive and compound data types
- `control_flow` - If, loops, match expressions
- `functions` - Function definition and calling
- `strings` - String handling and manipulation
- `error_handling` - Result, Option, and error patterns
- `ownership` - Ownership, borrowing, and lifetimes
- `structs` - Struct definition and methods
- `enums` - Enum variants and pattern matching
- `traits` - Trait definition and implementation
- `generics` - Generic functions and types
- `collections` - Vec, HashMap, and other collections
- `concurrency` - Threading and synchronization
- `unsafe` - Unsafe Rust features
- `macros` - Declarative and procedural macros
- `async` - Async/await and futures
- `memory` - Memory management and allocation
- `performance` - Optimization techniques
- `compiler` - Compiler internals and const evaluation

## Test Framework Architecture

### Core Components

1. **TestResult**: Individual test case results with timing
2. **TestSuite**: Groups related tests together
3. **TestRunner**: Orchestrates test execution
4. **TestSummary**: Aggregates results across all tests

### Macros

- `test_case!`: Creates timed test cases with panic handling
- `assert_with_msg!`: Assertions with custom error messages

### Example Usage

```rust
use rust_learning_path_tests::{TestSuite, test_case, assert_with_msg};

let mut suite = TestSuite::new("My Tests".to_string());

suite.add_test(test_case!("Addition Test", || {
    let result = 2 + 2;
    assert_with_msg!(result == 4, "Addition should work correctly");
}));
```

## Command Line Interface

### Options

- `-h, --help`: Show help message
- `-l, --level <LEVEL>`: Run tests for specific level
- `-c, --concept <CONCEPT>`: Run tests for specific concept
- `-v, --validate`: Validate all code examples
- `-s, --stats`: Show detailed statistics

### Examples

```bash
# Run all basic level tests
cargo run -- --level basic

# Test only ownership concepts
cargo run -- --concept ownership

# Get detailed statistics
cargo run -- --stats
```

## Continuous Integration

The test framework is designed for CI/CD integration:

```yaml
# Example GitHub Actions workflow
- name: Run Rust Learning Path Tests
  run: |
    cd rust-learning-path/test_framework
    cargo test
    python run_tests.py --quick
```

## Adding New Tests

### For New Code Examples

1. Add test cases to the appropriate level file (`basic_tests.rs`, etc.)
2. Use the `test_case!` macro for consistent error handling
3. Include clear assertions with descriptive messages
4. Test both success and failure cases where applicable

### For New Concepts

1. Create a new test function in the appropriate level file
2. Add the function to the level's test runner in `lib.rs`
3. Update the concept mapping in `run_concept_tests()`
4. Add documentation to this README

### Example New Test

```rust
pub fn test_new_concept() -> TestSuite {
    let mut suite = TestSuite::new("New Concept Tests".to_string());
    
    suite.add_test(test_case!("Basic Functionality", || {
        let result = new_concept_function();
        assert_with_msg!(result.is_ok(), "New concept should work correctly");
    }));
    
    suite.add_test(test_case!("Error Handling", || {
        let result = new_concept_function_with_error();
        assert_with_msg!(result.is_err(), "Error case should be handled");
    }));
    
    suite
}
```

## Test Reports

The framework generates detailed JSON reports containing:

- Overall test statistics
- Results by level and concept
- Individual test outcomes
- Performance metrics
- Validation status

Reports are saved as `test_report.json` in the project root.

## Troubleshooting

### Common Issues

1. **Build Failures**: Ensure Rust 1.70+ is installed
2. **Test Timeouts**: Individual tests timeout after 5 minutes
3. **Missing Dependencies**: Run `cargo build` to install dependencies
4. **Python Script Issues**: Ensure Python 3.7+ is available

### Debug Mode

Run tests with debug output:

```bash
RUST_LOG=debug cargo run -- --validate
```

### Performance Issues

If tests run slowly:

1. Use `--concept` to run specific test subsets
2. Check for infinite loops in test code
3. Monitor system resources during test execution

## Contributing

When contributing new tests:

1. Follow the existing test patterns
2. Include both positive and negative test cases
3. Use descriptive test names and error messages
4. Test edge cases and error conditions
5. Update documentation as needed

## License

This test framework is part of the Rust Learning Path project and follows the same licensing terms.