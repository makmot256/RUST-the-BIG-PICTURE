// implement the Clone trait for a struct
#[derive(Clone)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person1 = Person {
        name: String::from("John"),
        age: 30,
    };

    let person2 = person1.clone();

    println!("person1.name: {}, person1.age: {}", person1.name, person1.age);
    println!("person2.name: {}, person2.age: {}", person2.name, person2.age);
}
