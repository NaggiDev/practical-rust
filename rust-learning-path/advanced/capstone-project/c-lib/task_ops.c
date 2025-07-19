#include "task_ops.h"
#include <string.h>
#include <ctype.h>
#include <stdlib.h>

// Mathematical operations
int64_t fast_factorial(int32_t n) {
    if (n < 0 || n > 20) return -1; // Prevent overflow
    
    int64_t result = 1;
    for (int32_t i = 2; i <= n; i++) {
        result *= i;
    }
    return result;
}

int64_t fast_fibonacci(int32_t n) {
    if (n < 0) return -1;
    if (n <= 1) return n;
    
    int64_t a = 0, b = 1, temp;
    for (int32_t i = 2; i <= n; i++) {
        temp = a + b;
        a = b;
        b = temp;
    }
    return b;
}

double fast_sqrt(double x) {
    if (x < 0) return -1.0;
    if (x == 0) return 0.0;
    
    // Newton's method for square root
    double guess = x / 2.0;
    double prev_guess;
    
    do {
        prev_guess = guess;
        guess = (guess + x / guess) / 2.0;
    } while (guess != prev_guess);
    
    return guess;
}

int64_t fast_gcd(int64_t a, int64_t b) {
    if (a < 0) a = -a;
    if (b < 0) b = -b;
    
    while (b != 0) {
        int64_t temp = b;
        b = a % b;
        a = temp;
    }
    return a;
}

// Array operations
int64_t fast_array_sum(const int64_t* arr, size_t len) {
    if (!arr) return TASK_OPS_ERROR_NULL_POINTER;
    
    int64_t sum = 0;
    for (size_t i = 0; i < len; i++) {
        sum += arr[i];
    }
    return sum;
}

int64_t fast_array_max(const int64_t* arr, size_t len) {
    if (!arr || len == 0) return TASK_OPS_ERROR_NULL_POINTER;
    
    int64_t max = arr[0];
    for (size_t i = 1; i < len; i++) {
        if (arr[i] > max) {
            max = arr[i];
        }
    }
    return max;
}

// Simple quicksort implementation
static void quicksort(int64_t* arr, int low, int high) {
    if (low < high) {
        int64_t pivot = arr[high];
        int i = low - 1;
        
        for (int j = low; j < high; j++) {
            if (arr[j] <= pivot) {
                i++;
                int64_t temp = arr[i];
                arr[i] = arr[j];
                arr[j] = temp;
            }
        }
        
        int64_t temp = arr[i + 1];
        arr[i + 1] = arr[high];
        arr[high] = temp;
        
        int pi = i + 1;
        quicksort(arr, low, pi - 1);
        quicksort(arr, pi + 1, high);
    }
}

void fast_array_sort(int64_t* arr, size_t len) {
    if (!arr || len <= 1) return;
    quicksort(arr, 0, (int)len - 1);
}

// String operations
void fast_string_reverse(char* str, size_t len) {
    if (!str || len <= 1) return;
    
    for (size_t i = 0; i < len / 2; i++) {
        char temp = str[i];
        str[i] = str[len - 1 - i];
        str[len - 1 - i] = temp;
    }
}

void fast_string_uppercase(char* str, size_t len) {
    if (!str) return;
    
    for (size_t i = 0; i < len && str[i] != '\0'; i++) {
        str[i] = (char)toupper((unsigned char)str[i]);
    }
}

uint64_t fast_string_hash(const char* str, size_t len) {
    if (!str) return 0;
    
    // FNV-1a hash algorithm
    uint64_t hash = 14695981039346656037ULL;
    const uint64_t prime = 1099511628211ULL;
    
    for (size_t i = 0; i < len && str[i] != '\0'; i++) {
        hash ^= (uint64_t)(unsigned char)str[i];
        hash *= prime;
    }
    
    return hash;
}

// Memory operations
void fast_memory_copy(void* dest, const void* src, size_t len) {
    if (!dest || !src) return;
    memcpy(dest, src, len);
}

int fast_memory_compare(const void* a, const void* b, size_t len) {
    if (!a || !b) return TASK_OPS_ERROR_NULL_POINTER;
    return memcmp(a, b, len);
}