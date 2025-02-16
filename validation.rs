#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

impl User {
    fn new(name: String, age: u32) -> Result<User, String> {
        if age < 18 {
            Err("User must be at least 18 years old".to_string())
        } else {
            Ok(User { name, age })
        }
    }
}

fn main() {
    let user = User::new("John".to_string(), 17);
    match user {
        Ok(user) => println!("{:?}", user),
        Err(err) => println!("{}", err),
    }
}
