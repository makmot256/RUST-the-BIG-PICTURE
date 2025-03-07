fn main() {
    let name: Option<String> = Some(String::from("Alice"));
    let none_name: Option<String> = None;

    // Handling Option with match
    match name {
        Some(n) => println!("Name is: {}", n),
        None => println!("No name provided"),
    }

    // Using unwrap_or for Option
    let default_name = none_name.unwrap_or(String::from("Default Name"));
    println!("Name: {}", default_name);

    // Example with Result
    let age_result: Result<u32, String> = Ok(30);
    let error_result: Result<u32, String> = Err(String::from("Age not found"));

    // Handling Result with match
    match age_result {
        Ok(age) => println!("Age is: {}", age),
        Err(e) => println!("Error: {}", e),
    }

    // Using unwrap_or_else for Result
    let age = error_result.unwrap_or_else(|e| {
        println!("Error occurred: {}", e);
        0 // Default age
    });
    println!("Age: {}", age);
}

