fn factorial(n: u32) -> u32 {
    match n {
        0 | 1 => 1,
        _ => n * factorial(n - 1),
    }
}

fn main() {
    let number = 5;
    println!("The factorial of {} is {}", number, factorial(number));
}

