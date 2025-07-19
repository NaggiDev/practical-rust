#!/usr/bin/env python3
"""
Validation script for the Multi-threaded Web Scraper project.
This script checks that all required files are present and have the expected structure.
"""

import os
import sys
from pathlib import Path

def check_file_exists(filepath, description):
    """Check if a file exists and print status."""
    if os.path.exists(filepath):
        print(f"✓ {description}: {filepath}")
        return True
    else:
        print(f"✗ {description}: {filepath} (MISSING)")
        return False

def check_file_contains(filepath, content, description):
    """Check if a file contains specific content."""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            file_content = f.read()
            if content in file_content:
                print(f"✓ {description}")
                return True
            else:
                print(f"✗ {description} (MISSING)")
                return False
    except FileNotFoundError:
        print(f"✗ {description} (FILE NOT FOUND)")
        return False

def main():
    """Main validation function."""
    print("Multi-threaded Web Scraper Project Validation")
    print("=" * 50)
    
    project_root = Path(__file__).parent
    os.chdir(project_root)
    
    all_checks_passed = True
    
    # Check required files
    required_files = [
        ("README.md", "Project README"),
        ("Cargo.toml", "Cargo manifest"),
        ("src/main.rs", "Main entry point"),
        ("src/result.rs", "Result data structures"),
        ("src/scraper.rs", "Web scraper implementation"),
        ("src/worker.rs", "Worker thread implementation"),
        ("tests/integration_tests.rs", "Integration tests"),
        ("CONCEPTS.md", "Concept explanations"),
    ]
    
    for filepath, description in required_files:
        if not check_file_exists(filepath, description):
            all_checks_passed = False
    
    print("\nContent Validation:")
    print("-" * 30)
    
    # Check key content in files
    content_checks = [
        ("Cargo.toml", "reqwest", "HTTP client dependency"),
        ("Cargo.toml", "scraper", "HTML parsing dependency"),
        ("Cargo.toml", "clap", "CLI argument parsing dependency"),
        ("src/main.rs", "ThreadPoolScraper", "Multi-threaded scraper usage"),
        ("src/scraper.rs", "WebScraper", "Single-threaded scraper struct"),
        ("src/worker.rs", "Arc<Mutex<", "Shared state with Arc and Mutex"),
        ("src/worker.rs", "mpsc::", "Channel communication"),
        ("src/result.rs", "ScrapeResult", "Result data structure"),
        ("README.md", "Threading", "Threading concepts mentioned"),
        ("README.md", "Step-by-Step", "Step-by-step guide present"),
        ("CONCEPTS.md", "Arc", "Arc concept explanation"),
        ("CONCEPTS.md", "Mutex", "Mutex concept explanation"),
        ("CONCEPTS.md", "Channel", "Channel concept explanation"),
    ]
    
    for filepath, content, description in content_checks:
        if not check_file_contains(filepath, content, description):
            all_checks_passed = False
    
    print("\nProject Structure Validation:")
    print("-" * 35)
    
    # Check directory structure
    expected_dirs = ["src", "tests"]
    for dirname in expected_dirs:
        if os.path.isdir(dirname):
            print(f"✓ Directory: {dirname}")
        else:
            print(f"✗ Directory: {dirname} (MISSING)")
            all_checks_passed = False
    
    print("\nValidation Summary:")
    print("-" * 20)
    
    if all_checks_passed:
        print("✓ All validation checks passed!")
        print("The Multi-threaded Web Scraper project is properly structured.")
        return 0
    else:
        print("✗ Some validation checks failed.")
        print("Please review the missing components above.")
        return 1

if __name__ == "__main__":
    sys.exit(main())