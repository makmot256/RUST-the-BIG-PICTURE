// droptrait.rs
// Drop trait in Rust

// Define a struct that implements the Drop trait
struct Droppable {
    name: String,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Dropping {}", self.name);
    }
}

fn main() {
    {
        let _d1 = Droppable { name: "d1".to_string() };
        let _d2 = Droppable { name: "d2".to_string() };
        println!("End of block");
    }
    println!("End of main");
}

// Output:
// End of block
// Dropping d2
// Dropping d1
// End of main

// Define a struct that owns a Droppable
struct Container {
    droppable: Droppable,
}

impl Drop for Container {
    fn drop(&mut self) {
        println!("Dropping Container");
    }
}

fn main() {
    let _c1 = Container { droppable: Droppable { name: "c1".to_string() } };
    let _c2 = Container { droppable: Droppable { name: "c2".to_string() } };
    println!("End of block");
}

// Output:
// End of block
// Dropping Container
// Dropping c2
// Dropping Container
// Dropping c1

// Define a struct that owns a vector of Droppables
struct Container2 {
    droppables: Vec<Droppable>,
}

impl Drop for Container2 {
    fn drop(&mut self) {
        println!("Dropping Container2");
        for d in &self.droppables {
            println!("Dropping {}", d.name);
        }
    }
}

fn main() {
    let _c1 = Container2 {
        droppables: vec![
            Droppable { name: "c1-1".to_string() },
            Droppable { name: "c1-2".to_string() },
        ],
    };
    let _c2 = Container2 {
        droppables: vec![
            Droppable { name: "c2-1".to_string() },
            Droppable { name: "c2-2".to_string() },
        ],
    };
    println!("End of block");
}
// Output:
// End of block
// Dropping Container2
// Dropping c2-2
// Dropping c2-1
// Dropping Container2
// Dropping c1-2
// Dropping c1-1







