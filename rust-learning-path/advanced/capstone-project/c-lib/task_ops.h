#ifndef TASK_OPS_H
#define TASK_OPS_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

// Mathematical operations
int64_t fast_factorial(int32_t n);
int64_t fast_fibonacci(int32_t n);
double fast_sqrt(double x);
int64_t fast_gcd(int64_t a, int64_t b);

// Array operations
int64_t fast_array_sum(const int64_t* arr, size_t len);
int64_t fast_array_max(const int64_t* arr, size_t len);
void fast_array_sort(int64_t* arr, size_t len);

// String operations
void fast_string_reverse(char* str, size_t len);
void fast_string_uppercase(char* str, size_t len);
uint64_t fast_string_hash(const char* str, size_t len);

// Memory operations
void fast_memory_copy(void* dest, const void* src, size_t len);
int fast_memory_compare(const void* a, const void* b, size_t len);

// Error codes
#define TASK_OPS_SUCCESS 0
#define TASK_OPS_ERROR_NULL_POINTER -1
#define TASK_OPS_ERROR_INVALID_SIZE -2
#define TASK_OPS_ERROR_OVERFLOW -3

#ifdef __cplusplus
}
#endif

#endif // TASK_OPS_H