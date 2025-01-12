fn main() {
    // create an iterator over the numbers from 1 to 10
    let numbers = (1..11);

    // iterate over the numbers and print each one
    for n in numbers{
        println!("{}", n);
    }
    
    // show iterator chaining
    let numbers = (1..11);
    let doubles = numbers.map(|n| n * 2);
    let sum = doubles.sum();
    println!("Sum of doubles: {}", sum);

    // show lazy execution
    let numbers = (1..11);
    let doubles = numbers.map(|n| {
        println!("Multiplying {} by 2", n);
        n * 2
    });
    let sum = doubles.sum();
    println!("Sum of doubles: {}", sum);
}
