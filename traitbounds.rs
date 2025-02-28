// Define a trait with a method
trait Describable {
    fn describe(&self) -> String;
}

// Implement the trait for a struct
struct Animal {
    name: String,
}

impl Describable for Animal {
    fn describe(&self) -> String {
        format!("This is an animal named {}", self.name)
    }
}

// Function that uses a trait bound to accept any type that implements Describable
fn print_description<T: Describable>(item: T) {
    println!("{}", item.describe());
}

fn main() {
    let animal = Animal {
        name: String::from("Lion"),
    };

    print_description(animal);
}

