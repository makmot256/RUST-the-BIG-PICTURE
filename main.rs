fn main() {
    let a = 38;
    let b = 4;
    let sum = a + b;
    println!("Sum: {}", a, b );
}
// fn add(a: i32, b: i32) -> i32 {
//     return a + b;
// }
fn main() {
    let a = 38;
    let b = 4;
    let sum = add(a, b);
    println!("Sum: {}", sum);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

// The `main` function is the entry point for our program
// It is a special function that the operating system calls when the program is run
fn main() {
    // The `let` keyword is used to declare a new variable
    // The `a` variable is declared with the value of 38
    let a = 38;
    // The `b` variable is declared with the value of 4
    let b = 4;
    // The `sum` variable is declared and is assigned the result of calling the `add` function with the arguments `a` and `b`
    let sum = add(a, b);
    // The `println!` macro is used to print the value of the `sum` variable to the console
    println!("Sum: {}", sum);
}

// The `add` function takes two `i32` arguments, `a` and `b`, and returns their sum
fn add(a: i32, b: i32) -> i32 {
    // The `a + b` expression is evaluated and the result is returned from the function
    a + b
}


