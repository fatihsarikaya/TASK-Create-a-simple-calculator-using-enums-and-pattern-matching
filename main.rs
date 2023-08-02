use std::io;

// Create an enum called Operation with variants Add, Subtract, Multiply, and Divide.
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

// Create a function called calculate that takes an Operation enum as an argument and returns an f64 result.
fn calculate(operation: &Operation) -> f64 {
    match operation {
        // Implement the calculate function using pattern matching to perform the appropriate arithmetic operation based on the variant of the Operation enum.
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => a / b,
    }
}

fn main() {
    // Prompt the user to input the first number, the operation to be performed, and the second number.
    println!("Enter the first number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    let first_number: f64 = input.trim().parse().expect("Invalid input. Please enter a valid number.");

    println!("Enter the operation (+, -, *, /):");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    let operation_symbol = input.trim();

    println!("Enter the second number:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    let second_number: f64 = input.trim().parse().expect("Invalid input. Please enter a valid number.");

    // Create an Operation enum instance with the parsed input values.
    let operation = match operation_symbol {
        "+" => Operation::Add(first_number, second_number),
        "-" => Operation::Subtract(first_number, second_number),
        "*" => Operation::Multiply(first_number, second_number),
        "/" => Operation::Divide(first_number, second_number),
        _ => {
            println!("Invalid operation. Please enter one of +, -, *, /.");
            return;
        }
    };

    // Call the calculate function with the created Operation enum instance.
    let result = calculate(&operation);

    // Print the result to the console.
    println!("Result: {}", result);
}
