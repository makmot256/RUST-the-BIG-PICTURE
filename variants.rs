// variants.rs

// Variants in Rust

// Define an enum with variants
enum IpAddr {
    V4(String),
    V6(String),
}

// Implement methods on the enum
impl IpAddr {
    fn new(ip: String) -> IpAddr {
        if ip.contains(":") {
            IpAddr::V6(ip)
        } else {
            IpAddr::V4(ip)
        }
    }
}

// Use the enum
fn main() {
    let v4 = IpAddr::new("192.168.1.1".to_string());
    let v6 = IpAddr::new("2001:0db8:85a3:0000:0000:8a2e:0370:7334".to_string());

    match v4 {
        IpAddr::V4(ip) => println!("v4: {}", ip),
        IpAddr::V6(ip) => println!("v6: {}", ip),
    }

    match v6 {
        IpAddr::V4(ip) => println!("v4: {}", ip),
        IpAddr::V6(ip) => println!("v6: {}", ip),
    }
}
