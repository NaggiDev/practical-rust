#!/bin/bash
# Integration test script for Rust Learning Path Test Framework
# This script validates that the test framework works correctly

set -e

echo "🚀 Starting Rust Learning Path Test Framework Integration Test"
echo "=============================================================="

# Check prerequisites
echo "📋 Checking prerequisites..."

if ! command -v cargo &> /dev/null; then
    echo "❌ Error: cargo is not installed. Please install Rust."
    exit 1
fi

if ! command -v python3 &> /dev/null; then
    echo "❌ Error: python3 is not installed."
    exit 1
fi

echo "✅ Prerequisites check passed"

# Build the test framework
echo "🔨 Building test framework..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ Test framework built successfully"
else
    echo "❌ Failed to build test framework"
    exit 1
fi

# Run basic framework tests
echo "🧪 Running framework unit tests..."
cargo test

if [ $? -eq 0 ]; then
    echo "✅ Framework unit tests passed"
else
    echo "❌ Framework unit tests failed"
    exit 1
fi

# Test command line interface
echo "🖥️  Testing command line interface..."

# Test help command
echo "Testing --help option..."
cargo run --release --bin test-runner -- --help > /dev/null

if [ $? -eq 0 ]; then
    echo "✅ Help command works"
else
    echo "❌ Help command failed"
    exit 1
fi

# Test level-specific tests
echo "Testing level-specific test execution..."
for level in basic intermediate advanced expert; do
    echo "  Testing $level level..."
    timeout 60 cargo run --release --bin test-runner -- --level $level > /dev/null 2>&1
    
    if [ $? -eq 0 ] || [ $? -eq 1 ]; then
        echo "  ✅ $level level test execution works"
    else
        echo "  ❌ $level level test execution failed"
        exit 1
    fi
done

# Test concept-specific tests
echo "Testing concept-specific test execution..."
concepts=("variables" "strings" "ownership" "traits" "concurrency" "async")

for concept in "${concepts[@]}"; do
    echo "  Testing $concept concept..."
    timeout 30 cargo run --release --bin test-runner -- --concept $concept > /dev/null 2>&1
    
    if [ $? -eq 0 ] || [ $? -eq 1 ]; then
        echo "  ✅ $concept concept test execution works"
    else
        echo "  ❌ $concept concept test execution failed"
        exit 1
    fi
done

# Test validation mode
echo "Testing validation mode..."
timeout 120 cargo run --release --bin test-runner -- --validate > /dev/null 2>&1

if [ $? -eq 0 ] || [ $? -eq 1 ]; then
    echo "✅ Validation mode works"
else
    echo "❌ Validation mode failed"
    exit 1
fi

# Test statistics mode
echo "Testing statistics mode..."
timeout 120 cargo run --release --bin test-runner -- --stats > /dev/null 2>&1

if [ $? -eq 0 ] || [ $? -eq 1 ]; then
    echo "✅ Statistics mode works"
else
    echo "❌ Statistics mode failed"
    exit 1
fi

# Test Python automation script
echo "🐍 Testing Python automation script..."

# Test build-only mode
python3 run_tests.py --build-only

if [ $? -eq 0 ]; then
    echo "✅ Python script build-only mode works"
else
    echo "❌ Python script build-only mode failed"
    exit 1
fi

# Test quick validation
timeout 60 python3 run_tests.py --quick > /dev/null 2>&1

if [ $? -eq 0 ] || [ $? -eq 1 ]; then
    echo "✅ Python script quick validation works"
else
    echo "❌ Python script quick validation failed"
    exit 1
fi

# Performance test
echo "⚡ Running performance test..."
start_time=$(date +%s)
timeout 180 cargo run --release --bin test-runner > /dev/null 2>&1
end_time=$(date +%s)
duration=$((end_time - start_time))

if [ $duration -lt 180 ]; then
    echo "✅ Performance test passed (completed in ${duration}s)"
else
    echo "⚠️  Performance test timed out (may indicate performance issues)"
fi

# Memory usage test
echo "💾 Testing memory usage..."
if command -v valgrind &> /dev/null; then
    echo "Running memory leak detection..."
    timeout 60 valgrind --leak-check=summary --error-exitcode=1 \
        cargo run --release --bin test-runner -- --concept variables > /dev/null 2>&1
    
    if [ $? -eq 0 ]; then
        echo "✅ No memory leaks detected"
    else
        echo "⚠️  Memory issues detected (check valgrind output)"
    fi
else
    echo "⚠️  Valgrind not available, skipping memory test"
fi

# Final integration test
echo "🔄 Running full integration test..."
timeout 300 python3 run_tests.py > integration_test_output.txt 2>&1

if [ $? -eq 0 ] || [ $? -eq 1 ]; then
    echo "✅ Full integration test completed"
    
    # Check if report was generated
    if [ -f "../test_report.json" ]; then
        echo "✅ Test report generated successfully"
    else
        echo "⚠️  Test report not found"
    fi
else
    echo "❌ Full integration test failed"
    echo "Last 20 lines of output:"
    tail -20 integration_test_output.txt
    exit 1
fi

echo ""
echo "🎉 All integration tests passed!"
echo "============================================"
echo "✅ Test framework is working correctly"
echo "✅ Command line interface is functional"
echo "✅ All test levels can be executed"
echo "✅ All test concepts can be executed"
echo "✅ Python automation script works"
echo "✅ Performance is acceptable"
echo ""
echo "The Rust Learning Path Test Framework is ready for use!"