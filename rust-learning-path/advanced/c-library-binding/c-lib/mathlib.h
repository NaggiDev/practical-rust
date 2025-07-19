#ifndef MATHLIB_H
#define MATHLIB_H

#include <stddef.h>
#include <stdint.h>

// Mathematical operations
int32_t add_numbers(int32_t a, int32_t b);
int32_t multiply_numbers(int32_t a, int32_t b);
uint64_t factorial(uint32_t n);

// String operations
// Returns 0 on success, -1 on error
int reverse_string(const char* input, char* output, size_t output_size);
int uppercase_string(const char* input, char* output, size_t output_size);
size_t string_length(const char* str);

// Array operations
int32_t sum_array(const int32_t* array, size_t length);
int32_t find_max(const int32_t* array, size_t length, int32_t* max_value);

// Memory allocation helpers
char* allocate_string(size_t size);
void free_string(char* str);

// Error handling
const char* get_last_error(void);

#endif // MATHLIB_H