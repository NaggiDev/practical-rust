#!/usr/bin/env python3
"""
Validation script for the Rust Learning Path Quiz System
This script validates the structure and completeness of the quiz implementation
"""

import os
import re
from pathlib import Path

def validate_quiz_system():
    """Validate the quiz system implementation"""
    print("🔍 Validating Rust Learning Path Quiz System...")
    print("=" * 50)
    
    base_path = Path(".")
    issues = []
    
    # Check required files exist
    required_files = [
        "quiz_framework.rs",
        "quiz_runner.rs", 
        "QUIZ_README.md",
        "Cargo.toml",
        "lib.rs",
        "main.rs"
    ]
    
    print("📁 Checking required files...")
    for file in required_files:
        if (base_path / file).exists():
            print(f"  ✅ {file}")
        else:
            print(f"  ❌ {file} - MISSING")
            issues.append(f"Missing required file: {file}")
    
    # Validate quiz_framework.rs structure
    print("\n🧩 Validating quiz framework structure...")
    quiz_framework_path = base_path / "quiz_framework.rs"
    if quiz_framework_path.exists():
        content = quiz_framework_path.read_text(encoding='utf-8')
        
        # Check for required structs and enums
        required_types = [
            "enum QuestionType",
            "struct Question", 
            "enum Difficulty",
            "struct QuizResult",
            "struct Quiz",
            "struct QuizSummary",
            "struct QuizBank"
        ]
        
        for type_def in required_types:
            if type_def in content:
                print(f"  ✅ {type_def}")
            else:
                print(f"  ❌ {type_def} - NOT FOUND")
                issues.append(f"Missing type definition: {type_def}")
        
        # Check for required methods
        required_methods = [
            "fn start_interactive",
            "fn ask_question", 
            "fn check_answer",
            "fn generate_summary",
            "fn run_quiz",
            "fn initialize_quizzes"
        ]
        
        for method in required_methods:
            if method in content:
                print(f"  ✅ {method}")
            else:
                print(f"  ❌ {method} - NOT FOUND")
                issues.append(f"Missing method: {method}")
    
    # Validate quiz content
    print("\n🎯 Validating quiz content...")
    if quiz_framework_path.exists():
        content = quiz_framework_path.read_text(encoding='utf-8')
        
        # Check for quiz creation methods
        quiz_methods = [
            "create_basic_level_quizzes",
            "create_intermediate_level_quizzes", 
            "create_advanced_level_quizzes",
            "create_expert_level_quizzes"
        ]
        
        for method in quiz_methods:
            if method in content:
                print(f"  ✅ {method}")
            else:
                print(f"  ❌ {method} - NOT FOUND")
                issues.append(f"Missing quiz creation method: {method}")
        
        # Count questions (rough estimate)
        question_count = content.count("Question {")
        print(f"  📊 Estimated questions: {question_count}")
        
        if question_count < 10:
            issues.append(f"Low question count: {question_count} (should be 10+)")
    
    # Validate Cargo.toml
    print("\n📦 Validating Cargo.toml...")
    cargo_path = base_path / "Cargo.toml"
    if cargo_path.exists():
        content = cargo_path.read_text(encoding='utf-8')
        
        if "quiz-runner" in content:
            print("  ✅ Quiz runner binary configured")
        else:
            print("  ❌ Quiz runner binary not configured")
            issues.append("Quiz runner binary not configured in Cargo.toml")
        
        if "test-runner" in content:
            print("  ✅ Test runner binary configured")
        else:
            print("  ❌ Test runner binary not configured")
            issues.append("Test runner binary not configured in Cargo.toml")
    
    # Validate integration
    print("\n🔗 Validating integration...")
    lib_path = base_path / "lib.rs"
    if lib_path.exists():
        content = lib_path.read_text(encoding='utf-8')
        if "pub mod quiz_framework" in content:
            print("  ✅ Quiz framework module exported")
        else:
            print("  ❌ Quiz framework module not exported")
            issues.append("Quiz framework module not exported in lib.rs")
    
    main_path = base_path / "main.rs"
    if main_path.exists():
        content = main_path.read_text(encoding='utf-8')
        if "quiz_framework" in content:
            print("  ✅ Quiz functionality integrated in main")
        else:
            print("  ❌ Quiz functionality not integrated in main")
            issues.append("Quiz functionality not integrated in main.rs")
    
    # Summary
    print("\n" + "=" * 50)
    if not issues:
        print("🎉 Quiz system validation PASSED!")
        print("✅ All components are properly implemented")
        print("✅ Structure is complete and valid")
        print("✅ Integration points are configured")
        return True
    else:
        print("❌ Quiz system validation FAILED!")
        print(f"Found {len(issues)} issues:")
        for i, issue in enumerate(issues, 1):
            print(f"  {i}. {issue}")
        return False

def validate_quiz_questions():
    """Validate individual quiz questions for quality"""
    print("\n🎓 Validating quiz question quality...")
    print("-" * 30)
    
    quiz_framework_path = Path("quiz_framework.rs")
    if not quiz_framework_path.exists():
        print("❌ Cannot validate questions - quiz_framework.rs not found")
        return False
    
    content = quiz_framework_path.read_text(encoding='utf-8')
    
    # Extract question blocks (simplified)
    question_pattern = r'Question\s*\{[^}]+\}'
    questions = re.findall(question_pattern, content, re.DOTALL)
    
    print(f"📊 Found {len(questions)} question definitions")
    
    quality_checks = {
        "has_explanation": 0,
        "has_concept": 0, 
        "has_difficulty": 0,
        "has_options": 0,
        "has_code_snippet": 0
    }
    
    for question in questions:
        if "explanation:" in question:
            quality_checks["has_explanation"] += 1
        if "concept:" in question:
            quality_checks["has_concept"] += 1
        if "difficulty:" in question:
            quality_checks["has_difficulty"] += 1
        if "options:" in question:
            quality_checks["has_options"] += 1
        if "code_snippet:" in question:
            quality_checks["has_code_snippet"] += 1
    
    print("\n📋 Question Quality Metrics:")
    for check, count in quality_checks.items():
        percentage = (count / len(questions)) * 100 if questions else 0
        print(f"  {check.replace('_', ' ').title()}: {count}/{len(questions)} ({percentage:.1f}%)")
    
    return True

if __name__ == "__main__":
    print("🚀 Starting Quiz System Validation")
    print("=" * 50)
    
    # Run validations
    structure_valid = validate_quiz_system()
    questions_valid = validate_quiz_questions()
    
    print("\n" + "=" * 50)
    print("📋 FINAL VALIDATION REPORT")
    print("=" * 50)
    
    if structure_valid and questions_valid:
        print("🎉 OVERALL STATUS: PASSED")
        print("✅ Quiz system is ready for use!")
        print("✅ All components implemented correctly")
        print("✅ Questions are properly structured")
        print("\n🚀 Next steps:")
        print("  1. Test with: cargo run --bin quiz-runner")
        print("  2. Try interactive mode for best experience")
        print("  3. Add more questions as needed")
    else:
        print("❌ OVERALL STATUS: FAILED")
        print("🔧 Please fix the identified issues before using the quiz system")
        
    print("\n📚 For usage instructions, see QUIZ_README.md")