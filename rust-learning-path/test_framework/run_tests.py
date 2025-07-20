#!/usr/bin/env python3
"""
Test automation script for Rust Learning Path
This script runs all tests and generates reports
"""

import subprocess
import sys
import os
import json
import time
from pathlib import Path
from typing import Dict, List, Tuple

class TestRunner:
    def __init__(self, project_root: Path):
        self.project_root = project_root
        self.test_framework_dir = project_root / "test_framework"
        self.results = {}
        
    def run_command(self, cmd: List[str], cwd: Path = None) -> Tuple[int, str, str]:
        """Run a command and return exit code, stdout, stderr"""
        if cwd is None:
            cwd = self.test_framework_dir
            
        try:
            result = subprocess.run(
                cmd,
                cwd=cwd,
                capture_output=True,
                text=True,
                timeout=300  # 5 minute timeout
            )
            return result.returncode, result.stdout, result.stderr
        except subprocess.TimeoutExpired:
            return -1, "", "Command timed out after 5 minutes"
        except Exception as e:
            return -1, "", str(e)
    
    def build_test_framework(self) -> bool:
        """Build the test framework"""
        print("🔨 Building test framework...")
        
        exit_code, stdout, stderr = self.run_command(["cargo", "build", "--release"])
        
        if exit_code == 0:
            print("✅ Test framework built successfully")
            return True
        else:
            print("❌ Failed to build test framework")
            print(f"Error: {stderr}")
            return False
    
    def run_all_tests(self) -> Dict:
        """Run all tests and collect results"""
        print("🧪 Running all tests...")
        
        exit_code, stdout, stderr = self.run_command([
            "cargo", "run", "--release", "--bin", "test-runner", "--", "--stats"
        ])
        
        result = {
            "exit_code": exit_code,
            "stdout": stdout,
            "stderr": stderr,
            "success": exit_code == 0
        }
        
        # Parse test statistics from output
        if "Total tests:" in stdout:
            lines = stdout.split('\n')
            for line in lines:
                if "Total tests:" in line:
                    result["total_tests"] = int(line.split(':')[1].strip())
                elif "Passed:" in line and "%" in line:
                    parts = line.split(':')[1].strip().split()
                    result["passed_tests"] = int(parts[0])
                    result["pass_rate"] = float(parts[1].strip('()%'))
                elif "Failed:" in line and "%" in line:
                    parts = line.split(':')[1].strip().split()
                    result["failed_tests"] = int(parts[0])
        
        return result
    
    def run_level_tests(self, level: str) -> Dict:
        """Run tests for a specific level"""
        print(f"🎯 Running {level} level tests...")
        
        exit_code, stdout, stderr = self.run_command([
            "cargo", "run", "--release", "--bin", "test-runner", "--", "--level", level
        ])
        
        return {
            "level": level,
            "exit_code": exit_code,
            "stdout": stdout,
            "stderr": stderr,
            "success": exit_code == 0
        }
    
    def run_concept_tests(self, concept: str) -> Dict:
        """Run tests for a specific concept"""
        print(f"💡 Running {concept} concept tests...")
        
        exit_code, stdout, stderr = self.run_command([
            "cargo", "run", "--release", "--bin", "test-runner", "--", "--concept", concept
        ])
        
        return {
            "concept": concept,
            "exit_code": exit_code,
            "stdout": stdout,
            "stderr": stderr,
            "success": exit_code == 0
        }
    
    def validate_all_examples(self) -> Dict:
        """Validate all code examples"""
        print("✅ Validating all code examples...")
        
        exit_code, stdout, stderr = self.run_command([
            "cargo", "run", "--release", "--bin", "test-runner", "--", "--validate"
        ])
        
        return {
            "exit_code": exit_code,
            "stdout": stdout,
            "stderr": stderr,
            "success": exit_code == 0
        }
    
    def generate_report(self) -> None:
        """Generate a comprehensive test report"""
        print("📊 Generating test report...")
        
        report = {
            "timestamp": time.strftime("%Y-%m-%d %H:%M:%S"),
            "test_results": {}
        }
        
        # Run all tests
        all_tests_result = self.run_all_tests()
        report["test_results"]["all_tests"] = all_tests_result
        
        # Run tests by level
        levels = ["basic", "intermediate", "advanced", "expert"]
        report["test_results"]["by_level"] = {}
        
        for level in levels:
            level_result = self.run_level_tests(level)
            report["test_results"]["by_level"][level] = level_result
        
        # Run validation
        validation_result = self.validate_all_examples()
        report["test_results"]["validation"] = validation_result
        
        # Save report to file
        report_file = self.project_root / "test_report.json"
        with open(report_file, 'w') as f:
            json.dump(report, f, indent=2)
        
        print(f"📄 Test report saved to: {report_file}")
        
        # Print summary
        self.print_summary(report)
    
    def print_summary(self, report: Dict) -> None:
        """Print a summary of test results"""
        print("\n" + "="*50)
        print("TEST SUMMARY")
        print("="*50)
        
        all_tests = report["test_results"]["all_tests"]
        if all_tests["success"]:
            print("✅ Overall Status: PASSED")
        else:
            print("❌ Overall Status: FAILED")
        
        if "total_tests" in all_tests:
            print(f"📊 Total Tests: {all_tests['total_tests']}")
            print(f"✅ Passed: {all_tests.get('passed_tests', 0)}")
            print(f"❌ Failed: {all_tests.get('failed_tests', 0)}")
            print(f"📈 Pass Rate: {all_tests.get('pass_rate', 0):.1f}%")
        
        print("\nLevel Results:")
        for level, result in report["test_results"]["by_level"].items():
            status = "✅ PASS" if result["success"] else "❌ FAIL"
            print(f"  {level.capitalize()}: {status}")
        
        validation = report["test_results"]["validation"]
        validation_status = "✅ VALID" if validation["success"] else "❌ INVALID"
        print(f"\nCode Validation: {validation_status}")
        
        print("="*50)

def main():
    """Main entry point"""
    if len(sys.argv) > 1 and sys.argv[1] in ["-h", "--help"]:
        print("Rust Learning Path Test Automation")
        print("Usage: python run_tests.py [OPTIONS]")
        print("\nOptions:")
        print("  -h, --help     Show this help message")
        print("  --build-only   Only build the test framework")
        print("  --quick        Run quick validation only")
        print("  --full         Run full test suite with report (default)")
        return
    
    # Find project root
    script_dir = Path(__file__).parent
    project_root = script_dir.parent
    
    runner = TestRunner(project_root)
    
    # Build test framework first
    if not runner.build_test_framework():
        sys.exit(1)
    
    if len(sys.argv) > 1 and sys.argv[1] == "--build-only":
        print("✅ Build completed successfully")
        return
    
    if len(sys.argv) > 1 and sys.argv[1] == "--quick":
        # Quick validation
        result = runner.validate_all_examples()
        if result["success"]:
            print("✅ Quick validation passed")
        else:
            print("❌ Quick validation failed")
            sys.exit(1)
    else:
        # Full test suite
        runner.generate_report()

if __name__ == "__main__":
    main()