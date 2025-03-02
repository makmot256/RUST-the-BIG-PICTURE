fn main() {
    let mut x: u8 = 255;
    x += 1;
    println!("x is {}", x);
}
fn main() {
    let mut x: u8 = 255;
    x += 1;
    println!("x is {}", x);
    
    let y = 10;
    let z = y.wrapping_add(5);
    println!("y is {}, z is {}", y, z);

    let a = 10;
    let b = a.saturating_add(5);
    println!("a is {}, b is {}", a, b);
}
