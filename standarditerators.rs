fn main() {
    // create a vector of numbers from 1 to 10
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // iterate over the numbers and print each one
    for n in numbers.iter() {
        println!("{}", n);
    }

    // iterate over the numbers and print each one in reverse order
    for n in numbers.iter().rev() {
        println!("{}", n);
    }

    // iterate over the numbers and print each one with its index
    for (i, n) in numbers.iter().enumerate() {
        println!("{}: {}", i, n);
    }

    // iterate over the numbers and print each one with its index in reverse order
    for (i, n) in numbers.iter().rev().enumerate() {
        println!("{}: {}", i, n);
    }

    // iterate over the numbers and print each one if it is even
    for n in numbers.iter().filter(|x| **x % 2 == 0) {
        println!("{}", n);
    }

    // iterate over the numbers and print each one if it is odd
    for n in numbers.iter().filter(|x| **x % 2 != 0) {
        println!("{}", n);
    }

    // iterate over the numbers and print the sum of all the numbers
    let sum: i32 = numbers.iter().sum();
    println!("The sum of all the numbers is: {}", sum);
}
