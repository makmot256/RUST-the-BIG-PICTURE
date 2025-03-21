// Define a module
mod math_operations {
    // Function to add two numbers
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    // Function to subtract two numbers
    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }
}
// Use the module in the main function
fn main() {
    let sum = math_operations::add(10, 5);
    let difference = math_operations::subtract(10, 5);

    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
}