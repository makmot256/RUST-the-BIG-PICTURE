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
    // Define a function that takes two integers and returns true if the first is greater than the second
    fn is_greater_than(a: i32, b: i32) -> bool {
        a > b
    }

    // Define a function that takes two integers and returns their remainder
    fn remainder(a: i32, b: i32) -> Option<i32> {
        if b != 0 {
            Some(a % b)
        } else {
            None
        }
    }

    // Define a function that takes two integers and returns the maximum
    fn max(a: i32, b: i32) -> i32 {
        if a > b {
            a
        } else {
            b
        }
    }

    // Define a function that takes two integers and returns the minimum
    fn min(a: i32, b: i32) -> i32 {
        if a < b {
            a
        } else {
            b
        }
    }

    // Define a function that takes two integers and returns true if the first is less than the second
    fn is_less_than(a: i32, b: i32) -> bool {
        a < b
    }

    // Define a function that takes two integers and returns true if the first is equal to the second
    fn is_equal_to(a: i32, b: i32) -> bool {
        a == b
    }

    // Define a function that takes two integers and returns true if the first is greater than or equal to the second
    fn is_greater_than_or_equal_to(a: i32, b: i32) -> bool {
        a >= b
    }

    // Define a function that takes two integers and returns true if the first is less than or equal to the second
    fn is_less_than_or_equal_to(a: i32, b: i32) -> bool {
        a <= b
    }

    // Define a function that takes two integers and returns true if the first is not equal to the second
    fn is_not_equal_to(a: i32, b: i32) -> bool {
        a != b
    }

    // Define a function that takes an operator and two integers and returns the result of the operation

//calculator function
    fn calculate(op: char, a: i32, b: i32) -> i32 {
        match op {
            '+' => add(a, b),
            '-' => subtract(a, b),
            '*' => multiply(a, b),
            '/' => divide(a, b).expect("Cannot divide by zero"),
            _ => panic!("Invalid operator"),
        }
    }

1