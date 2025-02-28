// Define a macro that prints a message
macro_rules! say_hello {
    () => {
        println!("Hello, world!");
    };
}

// Define a macro that repeats a block of code n times
macro_rules! repeat {
    ($times:expr, $block:block) => {
        for _ in 0..$times {
            $block
        }
    };
}

fn main() {
    // Use the say_hello macro
    say_hello!();

    // Use the repeat macro to execute a block of code multiple times
    repeat!(3, {
        println!("This is a repeated message.");
    });
}

// Define a macro that prints a message
macro_rules! say_hello {
    () => {
        println!("Hello, world!");
    };
}

// Define a macro that repeats a block of code n times
macro_rules! repeat {
    ($times:expr, $block:block) => {
        for _ in 0..$times {
            $block
        }
    };
}

fn main() {
    // Use the say_hello macro
    say_hello!();

    // Use the repeat macro to execute a block of code multiple times
    repeat!(3, {
        println!("This is a repeated message.");
    });
}














