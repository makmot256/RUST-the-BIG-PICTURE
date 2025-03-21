// Copy trait in Rust

// Define a struct that implements the Copy trait
#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

// Implement the Copy trait for Point
impl Copy for Point {}

// Implement the Clone trait for Point
impl Clone for Point {
    fn clone(&self) -> Self {
        *self
    }
}

// Define a struct that does not implement the Copy trait
struct Person {
    name: String,
    age: u32,
}

// Implement the Clone trait for Person
impl Clone for Person {
    fn clone(&self) -> Self {
        Person {
            name: self.name.clone(),
            age: self.age,
        }
    }
}

// Show that Person implements the Clone trait
fn main2() {
    let p1 = Person {
        name: "John".to_string(),
        age: 30,
    };
    let p2 = p1.clone();

    println!("p1: {:?}", p1);
    println!("p2: {:?}", p2);

    // Show that p1 and p2 are equal
    assert_eq!(p1, p2);

    // Show that p1 and p2 have the same memory address
    println!("p1 addr: {:p}", &p1 as *const Person);
    println!("p2 addr: {:p}", &p2 as *const Person);
}

// Show that Point implements the Copy trait
fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1;

    println!("p1: {:?}", p1);
    println!("p2: {:?}", p2);

    // Show that p1 and p2 are equal
    assert_eq!(p1, p2);

    // Show that p1 and p2 have the same memory address
    println!("p1 addr: {:p}", &p1 as *const Point);
    println!("p2 addr: {:p}", &p2 as *const Point);

    // Show that p1 and p2 are not the same memory address
    assert_ne!(&p1 as *const Point, &p2 as *const Point);

    // Show that p1 and p2 are not the same memory address even after cloning
    let p3 = p1.clone();
    println!("p3 addr: {:p}", &p3 as *const Point);
    assert_ne!(&p1 as *const Point, &p3 as *const Point);

    // Show that p1 and p2 are not the same memory address even after cloning and moving
    let p4 = p1.clone();
    println!("p4 addr: {:p}", &p4 as *const Point);
    assert_ne!(&p1 as *const Point, &p4 as *const Point);
}




