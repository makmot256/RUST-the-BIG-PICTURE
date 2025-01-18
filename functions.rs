// Example demonstrating functions in Rust

// Define a function that takes two integers and returns their sum
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Define a function that takes two integers and returns their difference
fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

// Define a function that takes two integers and returns their product
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

// Define a function that takes two integers and returns their quotient
fn divide(a: i32, b: i32) -> Option<i32> {
    if b != 0 {
        Some(a / b)
    } else {
        None
    }
}

fn main() {
    let x = 10;
    let y = 5;

    let sum = add(x, y);
    let difference = subtract(x, y);
    let product = multiply(x, y);
    let quotient = divide(x, y);

    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);
    match quotient {
        Some(q) => println!("Quotient: {}", q),
        None => println!("Cannot divide by zero"),
    }
}

