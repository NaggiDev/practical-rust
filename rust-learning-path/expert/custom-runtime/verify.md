# Verification Instructions

To verify that the Custom Runtime project is working correctly, follow these steps:

## Prerequisites

Make sure you have Rust installed:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Running the Tests

1. Navigate to the project directory:
```bash
cd rust-learning-path/expert/custom-runtime
```

2. Run the unit tests:
```bash
cargo test
```

3. Run the integration tests specifically:
```bash
cargo test --test integration_tests
```

4. Run with output to see detailed behavior:
```bash
cargo test -- --nocapture
```

## Running the Example

Run the main example to see the runtime in action:
```bash
cargo run
```

## Expected Output

### Tests
All tests should pass. You should see output like:
```
running 15 tests
test task::tests::test_task_creation ... ok
test waker::tests::test_waker_creation ... ok
test executor::tests::test_executor_creation ... ok
...
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Example Run
The example should show:
- Simple async tasks executing
- Timer-based tasks with delays
- Cooperative multitasking with interleaved execution
- Complex async operations with shared state

## Troubleshooting

If tests fail:
1. Check that all dependencies are available
2. Ensure you're using a recent version of Rust (1.70+)
3. Review the error messages for specific issues

If the runtime seems to hang:
1. Check for infinite loops in timer polling
2. Verify that wakers are being called correctly
3. Look for tasks that never yield control

## Success Criteria

The implementation is successful if:
- [ ] All unit tests pass
- [ ] All integration tests pass  
- [ ] The example runs without panicking
- [ ] Timer tasks complete in roughly the expected time
- [ ] Multiple tasks can run concurrently
- [ ] The executor properly manages task lifecycle