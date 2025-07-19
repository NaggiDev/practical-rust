#include "mathlib.h"
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <errno.h>

// Global error message storage
static char last_error[256] = {0};

// Helper function to set error message
static void set_error(const char* message) {
    strncpy(last_error, message, sizeof(last_error) - 1);
    last_error[sizeof(last_error) - 1] = '\0';
}

// Mathematical operations
int32_t add_numbers(int32_t a, int32_t b) {
    return a + b;
}

int32_t multiply_numbers(int32_t a, int32_t b) {
    return a * b;
}

uint64_t factorial(uint32_t n) {
    if (n > 20) {
        set_error("Factorial input too large (max 20)");
        return 0;
    }
    
    uint64_t result = 1;
    for (uint32_t i = 2; i <= n; i++) {
        result *= i;
    }
    return result;
}

// String operations
int reverse_string(const char* input, char* output, size_t output_size) {
    if (!input || !output) {
        set_error("Null pointer passed to reverse_string");
        return -1;
    }
    
    size_t len = strlen(input);
    if (len + 1 > output_size) {
        set_error("Output buffer too small for reverse_string");
        return -1;
    }
    
    for (size_t i = 0; i < len; i++) {
        output[i] = input[len - 1 - i];
    }
    output[len] = '\0';
    
    return 0;
}

int uppercase_string(const char* input, char* output, size_t output_size) {
    if (!input || !output) {
        set_error("Null pointer passed to uppercase_string");
        return -1;
    }
    
    size_t len = strlen(input);
    if (len + 1 > output_size) {
        set_error("Output buffer too small for uppercase_string");
        return -1;
    }
    
    for (size_t i = 0; i < len; i++) {
        output[i] = toupper((unsigned char)input[i]);
    }
    output[len] = '\0';
    
    return 0;
}

size_t string_length(const char* str) {
    if (!str) {
        set_error("Null pointer passed to string_length");
        return 0;
    }
    return strlen(str);
}

// Array operations
int32_t sum_array(const int32_t* array, size_t length) {
    if (!array) {
        set_error("Null pointer passed to sum_array");
        return 0;
    }
    
    int32_t sum = 0;
    for (size_t i = 0; i < length; i++) {
        sum += array[i];
    }
    return sum;
}

int32_t find_max(const int32_t* array, size_t length, int32_t* max_value) {
    if (!array || !max_value) {
        set_error("Null pointer passed to find_max");
        return -1;
    }
    
    if (length == 0) {
        set_error("Empty array passed to find_max");
        return -1;
    }
    
    *max_value = array[0];
    for (size_t i = 1; i < length; i++) {
        if (array[i] > *max_value) {
            *max_value = array[i];
        }
    }
    
    return 0;
}

// Memory allocation helpers
char* allocate_string(size_t size) {
    char* str = malloc(size);
    if (!str) {
        set_error("Failed to allocate memory for string");
    }
    return str;
}

void free_string(char* str) {
    if (str) {
        free(str);
    }
}

// Error handling
const char* get_last_error(void) {
    return last_error;
}