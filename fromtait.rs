// formtrait.rs
// A form trait in Rust

// Define a trait with a method
trait Form {
    fn submit(&self) -> Result<(), String>;
}

// Define a struct that implements the Form trait
struct Person {
    name: String,
    age: u32,
}

impl Form for Person {
    fn submit(&self) -> Result<(), String> {
        if self.age < 18 {
            Err("User must be at least 18 years old".to_string())
        } else {
            Ok(())
        }
    }
}

// Define a struct that implements the Form trait
struct Address {
    street: String,
    city: String,
    state: String,
    zip: String,
}

impl Form for Address {
    fn submit(&self) -> Result<(), String> {
        if self.street.is_empty() || self.city.is_empty() || self.state.is_empty() || self.zip.is_empty() {
            Err("Address must have a street, city, state, and zip".to_string())
        } else {
            Ok(())
        }
    }
}

// Define a struct that implements the Form trait
struct Email {
    address: String,
    subject: String,
    body: String,
}

impl Form for Email {
    fn submit(&self) -> Result<(), String> {
        if self.address.is_empty() || self.subject.is_empty() || self.body.is_empty() {
            Err("Email must have an address, subject, and body".to_string())
        } else {
            Ok(())
        }
    }
}

// Define a struct that implements the Form trait
struct Product {
    name: String,
    price: f64,
}

impl Form for Product {
    fn submit(&self) -> Result<(), String> {
        if self.name.is_empty() || self.price <= 0.0 {
            Err("Product must have a name and price greater than zero".to_string())
        } else {
            Ok(())
        }
    }
}

// Define a struct that implements the Form trait
struct Order {
    customer: String,
    products: Vec<Product>,
}

impl Form for Order {
    fn submit(&self) -> Result<(), String> {
        if self.customer.is_empty() || self.products.is_empty() {
            Err("Order must have a customer and at least one product".to_string())
        } else {
            Ok(())
        }
    }
}

// Example demonstrating how to create a form and validate it
fn main() {
    let mut form = Person {
        name: "John".to_string(),
        age: 25,
    };

    match form.submit() {
        Ok(_) => println!("Form is valid"),
        Err(err) => println!("{}", err),
    }

    let mut form = Address {
        street: "123 Main Street".to_string(),
        city: "Anytown".to_string(),
        state: "CA".to_string(),
        zip: "12345".to_string(),
    };

    match form.submit() {
        Ok(_) => println!("Form is valid"),
        Err(err) => println!("{}", err),
    }

    let mut form = Email {
        address: "john@example.com".to_string(),
        subject: "Hello".to_string(),
        body: "This is an email".to_string(),
    };

    match form.submit() {
        Ok(_) => println!("Form is valid"),
        Err(err) => println!("{}", err),
    }

    let mut form = Product {
        name: "Apple".to_string(),
        price: 0.99,
    };

    match form.submit() {
        Ok(_) => println!("Form is valid"),
        Err(err) => println!("{}", err),
    }

    let mut form = Order {
        customer: "John".to_string(),
        products: vec![
            Product {
                name: "Apple".to_string(),
                price: 0.99,
            },
            Product {
                name: "Banana".to_string(),
                price: 0.49,
            },
        ],
    };

    match form.submit() {
        Ok(_) => println!("Form is valid"),
        Err(err) => println!("{}", err),
    }
}






