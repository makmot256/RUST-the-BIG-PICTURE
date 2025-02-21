fn main() {
    let a = 10u8;
    let b = a as i16;
    println!("a: {}, b: {}", a, b);

    let c = 10.5f64;
    let d = c as u8;
    println!("c: {}, d: {}", c, d);

    let e = 'a' as u8;
    println!("e: {}", e);
}
