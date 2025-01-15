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


    // show that iterators can be infinite
    let numbers = (1..).map(|n| n * 2);
    let sum = numbers.take(10).sum();
    println!("Sum of first 10 doubles: {}", sum);

    // show that you can use collect to turn an iterator into a collection
    let numbers = (1..10).collect::<Vec<i32>>();
    println!("{:?}", numbers);

    // show that you can use filter to filter out elements of an iterator
    let numbers = (1..10).filter(|n| *n % 2 == 0);
    let sum = numbers.sum();
    println!("Sum of even numbers: {}", sum);

    // show that you can use flat_map to flatten an iterator of iterators
    let numbers = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
    let numbers = numbers.into_iter().flat_map(|n| n.into_iter());
    for n in numbers{
        println!("{}", n);
    }

    // show that you can use take_while to take elements from an iterator until a condition is met
    let numbers = (1..).take_while(|n| *n < 10);
    for n in numbers{
        println!("{}", n);
    }

    // show that you can use skip_while to skip elements from an iterator until a condition is met
    let numbers = (1..).skip_while(|n| *n < 10);
    let sum = numbers.take(10).sum();
    println!("Sum of first 10 numbers greater than 10: {}", sum);

    // show that you can use chain to chain two iterators together
    let numbers = (1..10).chain(10..20);
    for n in numbers{
        println!("{}", n);
    }

    // show that you can use zip to combine two iterators into an iterator of tuples
    let numbers = (1..10).zip(10..20);
    for (n1, n2) in numbers{
        println!("({}, {})", n1, n2);
    }

    // show that you can use enumerate to iterate over an iterator and its index
    let numbers = (1..10).enumerate();
    for (i, n) in numbers{
        println!("{}: {}", i, n);
    }

    // show that you can use peekable to peek at the next element of an iterator
    let numbers = (1..10).peekable();
    println!("Next: {}", numbers.peek().unwrap());
    for n in numbers{
        println!("{}", n);
    }

    // show that you can use position to find the position of an element in an iterator
    let numbers = (1..10);
    let pos = numbers.position(|n| *n == 5);
    println!("5 is at position: {:?}", pos);

    // show that you can use any to find if any element of an iterator is true
    let numbers = (1..10);
    let any = numbers.any(|n| *n == 5);
    println!("Any element is 5: {}", any);

    // show that you can use all to find if all elements of an iterator are true
    let numbers = (1..10);
    let all = numbers.all(|n| *n < 10);
    println!("All elements are less than 10: {}", all);

    // show that you can use find to find an element in an iterator
    let numbers = (1..10);
    let find = numbers.find(|n| *n == 5);
    println!("Find 5: {:?}", find);

    // show that you can use find_map to find an element in an iterator and apply a function to it
    let numbers = (1..10);
    let find_map = numbers.find_map(|n| if *n == 5 { Some(n * 2) } else { None });
    println!("Find map 5: {:?}", find_map);

    // show that you can use max to find the maximum element in an iterator
    let numbers = (1..10);
    let max = numbers.max();
    println!("Max: {:?}", max);

    // show that you can use min to find the minimum element in an iterator
    let numbers = (1..10);
    let min = numbers.min();
    println!("Min: {:?}", min);

    // show that you can use fold to reduce an iterator to a single element
    let numbers = (1..10);
    let sum = numbers.fold(0, |acc, n| acc + n);
    println!("Sum: {}", sum);

    // show that you can use for_each to iterate over an iterator and apply a function to each element
    let numbers = (1..10);
    numbers.for_each(|n| println!("{}", n));

    // show that you can use partition to divide an iterator into two separate iterators
    let numbers = (1..10);
    let (even, odd) = numbers.partition(|n| *n % 2 == 0);
    println!("Even: {:?}", even);
    println!("Odd: {:?}", odd);

    // show that you can use cycle to cycle an iterator to make it infinite
    let numbers = (1..10).cycle();
    for n in numbers.take(20){
        println!("{}", n);
    }

    // show that you can use is_sorted to find if an iterator is sorted
    let numbers = (1..10);
    let sorted = numbers.is_sorted();
    println!("Is sorted: {}", sorted);

    // show that you can use is_sorted_by to find if an iterator is sorted by a specific closure
    let numbers = (1..10);
    let sorted = numbers.is_sorted_by(|n| *n % 2);
    println!("Is sorted by odd/even: {}", sorted);

    // show that you can use is_sorted_by_key to find if an iterator is sorted by a specific key
    let numbers = (1..10);
    let sorted = numbers.is_sorted_by_key(|n| *n % 2);
    println!("Is sorted by odd/even: {}", sorted);















