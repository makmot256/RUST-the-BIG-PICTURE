// enums.rs

// Define an enum in Rust
enum IpAddr {
    V4(String),
    V6(String),
}

// Implement methods on the enum
impl IpAddr {
    fn call(&self) {
        match self {
            IpAddr::V4(ip) => println!("Calling V4 address: {}", ip),
            IpAddr::V6(ip) => println!("Calling V6 address: {}", ip),
        }
    }
}

fn main() {
    let home = IpAddr::V4("127.0.0.1".to_string());
    let loopback = IpAddr::V6("::1".to_string());

    home.call();
    loopback.call();
}
