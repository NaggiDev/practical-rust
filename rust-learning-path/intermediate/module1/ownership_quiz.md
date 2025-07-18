# Advanced Ownership Quiz

Test your understanding of Rust's advanced ownership concepts with this quiz. Try to answer each question before looking at the answers.

## Questions

### 1. Borrowing Rules

Which of the following code snippets will compile successfully?

A)
```rust
fn main() {
    let mut value = 5;
    let r1 = &value;
    let r2 = &mut value;
    println!("{}, {}", r1, r2);
}
```

B)
```rust
fn main() {
    let mut value = 5;
    let r1 = &value;
    println!("{}", r1);
    let r2 = &mut value;
    *r2 += 1;
    println!("{}", r2);
}
```

C)
```rust
fn main() {
    let mut value = 5;
    let r1 = &mut value;
    let r2 = &mut value;
    *r1 += 1;
    *r2 += 1;
    println!("{}", value);
}
```

D)
```rust
fn main() {
    let value = 5;
    let r1 = &value;
    let r2 = &value;
    println!("{}, {}", r1, r2);
}
```

### 2. Lifetimes

What will the following code do?

```rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("short");
        result = longest(&string1, &string2);
    }
    println!("The longest string is: {}", result);
}
```

A) It will compile and print "The longest string is: long string is long"
B) It will compile but panic at runtime
C) It will not compile because string2 doesn't live long enough
D) It will not compile because the lifetime of y is not specified

### 3. Smart Pointers

What is the output of the following code?

```rust
use std::rc::Rc;

fn main() {
    let a = Rc::new(String::from("hello"));
    println!("count after creating a: {}", Rc::strong_count(&a));
    
    let b = Rc::clone(&a);
    println!("count after creating b: {}", Rc::strong_count(&a));
    
    {
        let c = Rc::clone(&a);
        println!("count after creating c: {}", Rc::strong_count(&a));
    }
    
    println!("count after c goes out of scope: {}", Rc::strong_count(&a));
}
```

A)
```
count after creating a: 1
count after creating b: 2
count after creating c: 3
count after c goes out of scope: 2
```

B)
```
count after creating a: 0
count after creating b: 1
count after creating c: 2
count after c goes out of scope: 1
```

C)
```
count after creating a: 1
count after creating b: 1
count after creating c: 1
count after c goes out of scope: 1
```

D)
```
count after creating a: 1
count after creating b: 2
count after creating c: 2
count after c goes out of scope: 2
```

### 4. Interior Mutability

What will the following code do?

```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);
    
    let mut borrow1 = data.borrow_mut();
    *borrow1 = 6;
    
    let borrow2 = data.borrow();
    println!("borrow1: {}, borrow2: {}", borrow1, borrow2);
}
```

A) It will compile and print "borrow1: 6, borrow2: 6"
B) It will compile but panic at runtime with a "already borrowed" error
C) It will not compile because you can't have a mutable and immutable borrow at the same time
D) It will compile and print "borrow1: 6, borrow2: 5"

### 5. Weak References

What is the purpose of Weak<T> in Rust?

A) To create references that are checked at compile time instead of runtime
B) To create references that are faster but less safe than regular references
C) To prevent reference cycles that could cause memory leaks
D) To create references that can be shared between threads

### 6. Box<T>

When would you use Box<T> in Rust? (Select all that apply)

A) When you want to store data on the heap instead of the stack
B) When you have a recursive type that needs a known size at compile time
C) When you want to transfer ownership of a large amount of data without copying it
D) When you need shared ownership of a value

### 7. Ownership Transfer

What will the following code do?

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s1);
}
```

A) It will compile and print "hello"
B) It will compile but print an empty string
C) It will not compile because s1 has been moved to s2
D) It will compile but panic at runtime

### 8. Arc<T> and Mutex<T>

What is the correct way to share mutable data between threads in Rust?

A) Use Rc<RefCell<T>> to share mutable data
B) Use Arc<Mutex<T>> to share mutable data
C) Use Box<T> with the Send trait
D) Use global static variables with unsafe code

## Answers

<details>
<summary>Click to reveal answers</summary>

1. B - The code compiles because r1 is no longer used after its println!, so a mutable borrow can be created.

2. D - The code will not compile because the lifetime of y is not specified in the function signature.

3. A - The reference count increases with each clone and decreases when a reference goes out of scope.

4. B - It will compile but panic at runtime because you can't have both mutable and immutable borrows active at the same time with RefCell.

5. C - Weak<T> is used to prevent reference cycles that could cause memory leaks.

6. A, B, C - Box<T> is used for heap allocation, recursive types, and transferring ownership of large data. It doesn't provide shared ownership (that's what Rc<T> is for).

7. C - The code will not compile because s1 has been moved to s2 and is no longer valid.

8. B - Arc<Mutex<T>> is the correct way to share mutable data between threads in Rust.

</details>