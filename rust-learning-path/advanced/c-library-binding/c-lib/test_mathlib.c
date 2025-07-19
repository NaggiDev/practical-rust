#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include "mathlib.h"

void test_math_operations() {
    printf("Testing mathematical operations...\n");
    
    assert(add_numbers(5, 3) == 8);
    assert(multiply_numbers(4, 7) == 28);
    assert(factorial(5) == 120);
    assert(factorial(0) == 1);
    
    // Test error case
    assert(factorial(25) == 0); // Should fail and return 0
    printf("Math operations: PASSED\n");
}

void test_string_operations() {
    printf("Testing string operations...\n");
    
    char buffer[100];
    
    // Test reverse
    assert(reverse_string("hello", buffer, sizeof(buffer)) == 0);
    assert(strcmp(buffer, "olleh") == 0);
    
    // Test uppercase
    assert(uppercase_string("hello", buffer, sizeof(buffer)) == 0);
    assert(strcmp(buffer, "HELLO") == 0);
    
    // Test length
    assert(string_length("hello") == 5);
    assert(string_length("") == 0);
    
    printf("String operations: PASSED\n");
}

void test_array_operations() {
    printf("Testing array operations...\n");
    
    int32_t arr[] = {1, 2, 3, 4, 5};
    size_t len = sizeof(arr) / sizeof(arr[0]);
    
    assert(sum_array(arr, len) == 15);
    
    int32_t max_val;
    assert(find_max(arr, len, &max_val) == 0);
    assert(max_val == 5);
    
    printf("Array operations: PASSED\n");
}

void test_memory_operations() {
    printf("Testing memory operations...\n");
    
    char* str = allocate_string(100);
    assert(str != NULL);
    
    strcpy(str, "test string");
    assert(strcmp(str, "test string") == 0);
    
    free_string(str);
    
    printf("Memory operations: PASSED\n");
}

void test_error_handling() {
    printf("Testing error handling...\n");
    
    // Test factorial error
    factorial(25);
    const char* error = get_last_error();
    assert(strlen(error) > 0);
    printf("Last error: %s\n", error);
    
    // Test null pointer error
    char buffer[10];
    assert(reverse_string(NULL, buffer, sizeof(buffer)) == -1);
    error = get_last_error();
    assert(strlen(error) > 0);
    printf("Last error: %s\n", error);
    
    printf("Error handling: PASSED\n");
}

int main() {
    printf("=== C Library Test Suite ===\n\n");
    
    test_math_operations();
    test_string_operations();
    test_array_operations();
    test_memory_operations();
    test_error_handling();
    
    printf("\n=== All Tests Passed! ===\n");
    return 0;
}