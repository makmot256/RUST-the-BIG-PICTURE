fn main() {
    // Example 1: Simple if statement.
    let number = 5;
    if number < 10 {
        println!("Example 1: {} is less than 10.", number);
    }

    // Example 2: if-else statement.
    let number = 12;
    if number % 2 == 0 {
        println!("Example 2: {} is even.", number);
    } else {
        println!("Example 2: {} is odd.", number);
    }

    // Example 3: if-else-if chain.
    let num = 15;
    if num < 10 {
        println!("Example 3: {} is less than 10.", num);
    } else if num < 20 {
        println!("Example 3: {} is between 10 and 20.", num);
    } else {
        println!("Example 3: {} is 20 or greater.", num);
    }

    // Example 4: Nested if statements.
    let age = 25;
    if age >= 18 {
        println!("Example 4: You are an adult.");
        if age >= 21 {
            println!("Example 4: You can legally drink in some countries.");
        } else {
            println!("Example 4: You cannot legally drink in some countries.");
        }
    } else {
        println!("Example 4: You are a minor.");
    }

    // Example 5: Using if in a let assignment.
    let flag = if true { "Yes" } else { "No" };
    println!("Example 5: The flag is {}.", flag);
}