//! Test case with good code that should pass all lints

use compiler_plugin::{lint_function, LintableStruct};

#[lint_function]
fn well_named_function() {
    let _used_variable = 42;
    println!("This function follows good practices");
}

#[lint_function]
fn simple_calculation(x: i32, y: i32) -> i32 {
    let result = x + y;
    result
}

#[derive(LintableStruct)]
struct WellNamedStruct {
    field_one: i32,
    field_two: String,
    field_three: bool,
}

#[derive(LintableStruct)]
struct SimpleStruct {
    value: i32,
}

impl WellNamedStruct {
    fn new(field_one: i32, field_two: String, field_three: bool) -> Self {
        Self {
            field_one,
            field_two,
            field_three,
        }
    }

    fn get_field_one(&self) -> i32 {
        self.field_one
    }
}

fn main() {
    well_named_function();
    
    let result = simple_calculation(5, 10);
    println!("Result: {}", result);
    
    let instance = WellNamedStruct::new(
        42,
        "test".to_string(),
        true,
    );
    
    println!("Field one: {}", instance.get_field_one());
    println!("Lint info: {}", instance.lint_info());
}