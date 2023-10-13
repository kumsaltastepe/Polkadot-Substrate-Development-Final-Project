use std::io;

// Define the Operation enum
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

// Implement the calculate function using pattern matching
fn calculate(operation: &Operation) -> f64 {
    match operation {
        Operation::Add(x, y) => x + y,
        Operation::Subtract(x, y) => x - y,
        Operation::Multiply(x, y) => x * y,
        Operation::Divide(x, y) => {
            if *y != 0.0 {
                x / y
            } else {
                println!("Error: Division by zero!");
                f64::NAN
            }
        }
    }
}

fn main() {
    // Prompt the user for input
    println!("Enter the first number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let first_number: f64 = input.trim().parse().expect("Invalid number");

    println!("Enter the operation (+, -, *, /):");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let operation_str = input.trim();

    println!("Enter the second number:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let second_number: f64 = input.trim().parse().expect("Invalid number");

    // Create an Operation enum instance
    let operation = match operation_str {
        "+" => Operation::Add(first_number, second_number),
        "-" => Operation::Subtract(first_number, second_number),
        "*" => Operation::Multiply(first_number, second_number),
        "/" => Operation::Divide(first_number, second_number),
        _ => {
            println!("Error: Invalid operation!");
            return;
        }
    };

    // Call the calculate function
    let result = calculate(&operation);

    // Print the result
    println!("Result: {}", result);
}
