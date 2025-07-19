#!/usr/bin/env python3
"""
Validation script for the Compiler Plugin project.

This script validates the project structure and code without requiring
Rust to be installed. It checks for:
- Correct file structure
- Presence of required files
- Basic syntax validation
- Documentation completeness
"""

import os
import sys
from pathlib import Path

def check_file_exists(path, description):
    """Check if a file exists and report the result."""
    if os.path.exists(path):
        print(f"✓ {description}: {path}")
        return True
    else:
        print(f"✗ {description}: {path} (missing)")
        return False

def check_file_content(path, required_content, description):
    """Check if a file contains required content."""
    try:
        with open(path, 'r', encoding='utf-8') as f:
            content = f.read()
            if all(req in content for req in required_content):
                print(f"✓ {description}: {path}")
                return True
            else:
                missing = [req for req in required_content if req not in content]
                print(f"✗ {description}: {path} (missing: {missing})")
                return False
    except FileNotFoundError:
        print(f"✗ {description}: {path} (file not found)")
        return False
    except Exception as e:
        print(f"✗ {description}: {path} (error: {e})")
        return False

def validate_project_structure():
    """Validate the overall project structure."""
    print("=== Project Structure Validation ===")
    
    required_files = [
        ("README.md", "Project README"),
        ("Cargo.toml", "Cargo configuration"),
        ("src/lib.rs", "Main library file"),
        ("src/lint.rs", "Lint implementation"),
        ("src/diagnostics.rs", "Diagnostics module"),
        ("tests/integration_tests.rs", "Integration tests"),
        ("tests/test_cases/good_code.rs", "Good code test case"),
        ("tests/test_cases/bad_code.rs", "Bad code test case"),
        ("examples/usage.rs", "Usage example"),
        ("CONCEPTS.md", "Concepts documentation"),
        (".gitignore", "Git ignore file"),
        ("build.rs", "Build script"),
    ]
    
    all_good = True
    for file_path, description in required_files:
        if not check_file_exists(file_path, description):
            all_good = False
    
    return all_good

def validate_cargo_toml():
    """Validate Cargo.toml configuration."""
    print("\n=== Cargo.toml Validation ===")
    
    required_content = [
        'proc-macro = true',
        'proc-macro2',
        'quote',
        'syn',
        'trybuild',
    ]
    
    return check_file_content("Cargo.toml", required_content, "Cargo.toml configuration")

def validate_lib_rs():
    """Validate the main library file."""
    print("\n=== lib.rs Validation ===")
    
    required_content = [
        '#[proc_macro_attribute]',
        '#[proc_macro_derive(',
        '#[proc_macro]',
        'pub fn lint_function',
        'pub fn derive_lintable_struct',
        'pub fn analyze_module',
    ]
    
    return check_file_content("src/lib.rs", required_content, "Main library implementation")

def validate_lint_rs():
    """Validate the lint implementation."""
    print("\n=== lint.rs Validation ===")
    
    required_content = [
        'pub struct FunctionLinter',
        'pub struct StructLinter',
        'pub struct ModuleLinter',
        'impl<\'ast> Visit<\'ast>',
        'fn analyze_function',
        'fn analyze_struct',
    ]
    
    return check_file_content("src/lint.rs", required_content, "Lint implementation")

def validate_diagnostics_rs():
    """Validate the diagnostics module."""
    print("\n=== diagnostics.rs Validation ===")
    
    required_content = [
        'pub enum DiagnosticLevel',
        'pub struct Diagnostic',
        'pub struct DiagnosticBuilder',
        'pub struct DiagnosticCollection',
        'pub fn emit',
    ]
    
    return check_file_content("src/diagnostics.rs", required_content, "Diagnostics implementation")

def validate_tests():
    """Validate test files."""
    print("\n=== Test Validation ===")
    
    test_files = [
        ("tests/integration_tests.rs", [
            '#[test]',
            'lint_function',
            'LintableStruct',
            'trybuild::TestCases',
        ]),
        ("tests/test_cases/good_code.rs", [
            'lint_function',
            'LintableStruct',
            'well_named_function',
        ]),
        ("tests/test_cases/bad_code.rs", [
            'BadFunctionName',
            'badStructName',
            'unused_variable',
        ]),
    ]
    
    all_good = True
    for file_path, required_content in test_files:
        if not check_file_content(file_path, required_content, f"Test file {file_path}"):
            all_good = False
    
    return all_good

def validate_documentation():
    """Validate documentation files."""
    print("\n=== Documentation Validation ===")
    
    doc_files = [
        ("README.md", [
            "Learning Objectives",
            "Step-by-Step Implementation",
            "Extension Challenges",
            "Testing Your Implementation",
        ]),
        ("CONCEPTS.md", [
            "Procedural Macros",
            "Compiler Architecture",
            "Abstract Syntax Trees",
            "Token Streams",
            "Diagnostic System",
        ]),
    ]
    
    all_good = True
    for file_path, required_content in doc_files:
        if not check_file_content(file_path, required_content, f"Documentation {file_path}"):
            all_good = False
    
    return all_good

def validate_example():
    """Validate the usage example."""
    print("\n=== Example Validation ===")
    
    required_content = [
        'lint_function',
        'LintableStruct',
        'analyze_module',
        'calculate_fibonacci',
        'UserAccount',
        'fn main()',
    ]
    
    return check_file_content("examples/usage.rs", required_content, "Usage example")

def main():
    """Main validation function."""
    print("Compiler Plugin Project Validation")
    print("=" * 50)
    
    # Change to the project directory
    os.chdir(Path(__file__).parent)
    
    validations = [
        validate_project_structure,
        validate_cargo_toml,
        validate_lib_rs,
        validate_lint_rs,
        validate_diagnostics_rs,
        validate_tests,
        validate_documentation,
        validate_example,
    ]
    
    all_passed = True
    for validation in validations:
        if not validation():
            all_passed = False
    
    print("\n" + "=" * 50)
    if all_passed:
        print("✓ All validations passed! The project structure is complete.")
        print("\nTo test the project with Rust installed, run:")
        print("  cargo test")
        print("  cargo run --example usage")
        return 0
    else:
        print("✗ Some validations failed. Please check the issues above.")
        return 1

if __name__ == "__main__":
    sys.exit(main())