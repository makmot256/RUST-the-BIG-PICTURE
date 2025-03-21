
fn main(){
    // create a closure that takes two parameters and returns their sum
    let add = |a, b| a + b;

    // create a closure that takes two parameters and returns the sum of all numbers from the first to the second
    let sum_range = |from,to|{
        // initialize a variable to hold the sum
        let mut sum = 0;

        // iterate over the range from..to and add each number to the sum
        for n in from..to{
            // use the add closure to add the current number to the sum
            sum = add(sum, n);
        }
        // return the sum
        sum
    }
    // print the sum of 1 to 10 using the sum_range closure
    println!("Sum of 1 to 10: {}", sum_range(1,11));
}

// Iterators in rust    
    // create an iterator over the numbers from 1 to 10
    let numbers = (1..11);
    let numbers = (2..22)

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

    //solana web3 js
    //anchor framewrok.


//foundry 
//solidityby example



