// Define a trait
trait DisplayName {
    fn display_name(&self) -> String;
}

// Define a struct in the same crate
struct Person {
    name: String,
}

// Implement the trait for the struct in the same crate
impl DisplayName for Person {
    fn display_name(&self) -> String {
        format!("Name: {}", self.name)
    }
}

// Define a struct in a different crate (simulated here)
struct ExternalType {
    id: u32,
}

// Attempt to implement the trait for the external struct in this crate
// This will result in a compilation error due to the orphan rule
// impl DisplayName for ExternalType {
//     fn display_name(&self) -> String {
//         format!("ID: {}", self.id)
//     }
// }

fn main() {
    let person = Person {
        name: String::from("Alice"),
    };
    println!("{}", person.display_name());

    // Uncommenting the below lines will cause a compilation error
    // let external = ExternalType { id: 42 };
    // println!("{}", external.display_name());
}

