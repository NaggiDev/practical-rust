// Example: Control Flow
fn main() {
    // If expressions
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    
    // If in a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
    
    // Loop expression
    let mut counter = 0;
    
    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2;  // Return a value from the loop
        }
    };
    
    println!("The result is: {}", result);
    
    // While loop
    let mut number = 3;
    
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    
    println!("LIFTOFF!!!");
    
    // For loop with array
    let a = [10, 20, 30, 40, 50];
    
    for element in a.iter() {
        println!("The value is: {}", element);
    }
    
    // For loop with range
    for number in 1..4 {  // 1, 2, 3
        println!("Range: {}", number);
    }
    
    // For loop with inclusive range
    for number in 1..=3 {  // 1, 2, 3
        println!("Inclusive range: {}", number);
    }
    
    // For loop with reverse
    for number in (1..4).rev() {  // 3, 2, 1
        println!("Countdown: {}", number);
    }
    
    // Match expression
    let dice_roll = 9;
    
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // Match any other value
        other => move_player(other),
    }
    
    // Match with multiple patterns
    let dice_roll = 9;
    
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // Match any other value and ignore it
        _ => reroll(),
    }
    
    // If let expression
    let some_value = Some(3);
    
    if let Some(3) = some_value {
        println!("three");
    }
}

fn add_fancy_hat() {
    println!("Add fancy hat");
}

fn remove_fancy_hat() {
    println!("Remove fancy hat");
}

fn move_player(num_spaces: u8) {
    println!("Move player {} spaces", num_spaces);
}

fn reroll() {
    println!("Reroll the dice");
}