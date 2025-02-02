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


fn ownership_example() {
    // Ownership: value is owned by variable
    let x = String::from("ownership");
    println!("x: {}", x); // x owns the string

    // Move: ownership is transferred
    let y = x; // x is no longer valid, y now owns the string
    // println!("x: {}", x); // this would cause a compile error
    println!("y: {}", y);

    // Clone: deep copy is created
    let z = y.clone(); // y and z both own separate copies of the string
    println!("y: {}, z: {}", y, z);
}

fn borrowing_example() {
    // Borrowing: allows temporary access without transfer of ownership
    let a = String::from("borrowing");

    let length = calculate_length(&a); // a is borrowed by calculate_length
    println!("The length of '{}' is {}.", a, length); // a can still be used

    // Mutable Borrowing: allows modification of the borrowed value
    let mut b = String::from("mutable");
    change(&mut b); // b is borrowed mutably by change
    println!("b after change: {}", b);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" borrow");
}

fn me_more_ownership_example() {
    // Slices: references to parts of a collection
    let s = String::from("me more ownership");
    let hello = &s[3..7]; // hello is a slice of s, references the characters "o m"
    println!("s: {}, hello: {}", s, hello);

    // Copy: types that implement Copy are copied when passed as arguments or assigned to new variables
    let x = 5;
    let y = x; // x is copied to y
    println!("x: {}, y: {}", x, y);

    let s = String::from("copy");
    let s2 = s.clone(); // s is copied to s2
    println!("s: {}, s2: {}", s, s2);

    // Move: types that do not implement Copy are moved when passed as arguments or assigned to new variables
    let s = String::from("move");
    let s2 = s; // s is moved to s2
    // println!("s: {}", s); // this would cause a compile error
    println!("s2: {}", s2);
}
fn more_ownership_example() {
    // Ownership rules:
    // 1. Each value in Rust has an owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.
    let s = String::from("ownership rules");
    {
        let s2 = s; // s is moved to s2, s is no longer valid
        println!("s2: {}", s2);
    }
    // println!("s: {}", s); // this would cause a compile error

    // References: allow multiple owners
    let s = String::from("references");
    let r1 = &s; // r1 is a reference to s
    let r2 = &s; // r2 is a reference to s
    println!("r1: {}, r2: {}", r1, r2);

    // Mutable references
    let mut s = String::from("mutable references");
    let r1 = &mut s; // r1 is a mutable reference to s
    // let r2 = &mut s; // this would cause a compile error
    println!("r1: {}", r1);
}
