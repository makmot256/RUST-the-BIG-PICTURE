fn main() {
    // Ownership example
    let s1 = String::from("hello"); // s1 owns the string
    let s2 = s1; // ownership of the string is moved to s2

    // println!("{}", s1); // this would cause a compile error because s1 no longer owns the string

    println!("{}", s2); // prints "hello"

    // Borrowing example
    let s3 = String::from("world");
    let len = calculate_length(&s3); // s3 is borrowed by calculate_length

    println!("The length of '{}' is {}.", s3, len); // s3 can still be used because it was borrowed, not moved
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

