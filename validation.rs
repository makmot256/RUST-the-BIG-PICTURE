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
/*************  ✨ Codeium Command ⭐  *************/
fn validate_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        Err("Name cannot be empty".to_string())
    } else if name.chars().all(char::is_alphabetic) {
        Ok(())
    } else {
        Err("Name must contain only alphabetic characters".to_string())
    }
}

fn main() {
    let user = User::new("John".to_string(), 17);
    match user {
        Ok(user) => {
            match validate_name(&user.name) {
                Ok(_) => println!("{:?}", user),
                Err(err) => println!("{}", err),
            }
        }
        Err(err) => println!("{}", err),
    }
}

/******  30afb6cf-1b05-476a-856b-32604c62c825  *******/a