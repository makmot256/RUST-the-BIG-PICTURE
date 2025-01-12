// Example demonstrating snake case notation in Rust

// Define a module using snake case
mod math_operations {
    // Define a function using snake case
    pub fn add_two_numbers(num_one: i32, num_two: i32) -> i32 {
        num_one + num_two
    }

    pub fn subtract_two_numbers(num_one: i32, num_two: i32) -> i32 {
        num_one - num_two
    }
}

fn main() {
    // Define variables using snake case
    let first_number = 10;
    let second_number = 5;

    // Call functions using snake case
    let sum = math_operations::add_two_numbers(first_number, second_number);
    let difference = math_operations::subtract_two_numbers(first_number, second_number);

    println!("The sum of {} and {} is {}", first_number, second_number, sum);
    println!(
        "The difference between {} and {} is {}",
        first_number, second_number, difference
    );
}
