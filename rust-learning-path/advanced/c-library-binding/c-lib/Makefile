# Makefile for mathlib C library

CC = gcc
CFLAGS = -Wall -Wextra -std=c99 -fPIC -O2
AR = ar
ARFLAGS = rcs

# Source files
SOURCES = mathlib.c
OBJECTS = $(SOURCES:.c=.o)
LIBRARY = libmathlib.a
SHARED_LIB = libmathlib.so

# Default target
all: $(LIBRARY) $(SHARED_LIB)

# Static library
$(LIBRARY): $(OBJECTS)
	$(AR) $(ARFLAGS) $@ $^

# Shared library
$(SHARED_LIB): $(OBJECTS)
	$(CC) -shared -o $@ $^

# Object files
%.o: %.c mathlib.h
	$(CC) $(CFLAGS) -c $< -o $@

# Clean build artifacts
clean:
	rm -f $(OBJECTS) $(LIBRARY) $(SHARED_LIB)

# Install (optional)
install: $(LIBRARY) $(SHARED_LIB)
	cp $(LIBRARY) /usr/local/lib/
	cp $(SHARED_LIB) /usr/local/lib/
	cp mathlib.h /usr/local/include/

# Test compilation
test: $(LIBRARY)
	$(CC) $(CFLAGS) -o test_mathlib test_mathlib.c -L. -lmathlib

.PHONY: all clean install test