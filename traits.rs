// Traits in Rust

// define a trait, a collection of methods or behaviors that a type can have
trait Animal {
    // define a method that any type implementing this trait must have
    fn sound(&self) -> String;
}

// implement the trait for a struct
struct Dog; // define a struct

impl Animal for Dog { // implement the Animal trait for Dog
    fn sound(&self) -> String { // define the sound method
        "Woof!".to_string() // return a string
    }
}

// use the trait
fn make_sound(animal: &dyn Animal) { // define a function that takes any type that implements Animal
    println!("{}", animal.sound()); // call the sound method on the passed type
}

fn main() {
    let dog = Dog; // create an instance of Dog
    make_sound(&dog); // pass the dog to the make_sound function
}



