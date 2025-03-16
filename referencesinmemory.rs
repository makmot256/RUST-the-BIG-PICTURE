// referencesinmemory.rs

// Rust references are a type of pointer that allows referring to the value of a variable
// without taking ownership of it. References are safe, because they are guaranteed to point
// to a valid value.

// References can be used to borrow the value of a variable, which is useful for passing
// arguments to functions or returning values from functions.

// References can also be used to create a shared ownership of a value, which is useful
// for avoiding the overhead of copying values.

// References can be used to create a mutable reference to a value, which is useful for
// modifying the value.

fn main() {
    let x = 5;
    let y = &x; // y is a reference to x

    println!("x: {}", x);
    println!("y: {}", y);

    let mut s = String::from("hello");
    let r = &s; // r is a reference to s
    let mr = &mut s; // mr is a mutable reference to s

    println!("r: {}", r);
    println!("mr: {}", mr);

    // References can be used to borrow the value of a variable, which is useful for passing
    // arguments to functions or returning values from functions.
    fn print_value(x: &i32) {
        println!("x: {}", x);
    }

    print_value(&x);

    // References can also be used to create a shared ownership of a value, which is useful
    // for avoiding the overhead of copying values.
    let s1 = String::from("hello");
    let s2 = &s1; // s2 is a reference to s1
    println!("s1: {}", s1);
    println!("s2: {}", s2);

    // References can be used to create a mutable reference to a value, which is useful for
    // modifying the value.
    let mut s = String::from("hello");
    let mr = &mut s; // mr is a mutable reference to s
    mr.push_str(", world");
    println!("s: {}", s);

    // References can be used to create a reference to a reference, which is useful for
    // creating a chain of references.
    let x = 5;
    let y = &x; // y is a reference to x
    let z = &y; // z is a reference to y
    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);

    // References can be used to create a mutable reference to a reference, which is useful
    // for modifying the value that the reference points to.
    let mut s = String::from("hello");
    let mr1 = &mut s; // mr1 is a mutable reference to s
    let mr2 = &mut mr1; // mr2 is a mutable reference to mr1
    mr2.push_str(", world");
    println!("s: {}", s);

    // References can be used to create a reference to a slice, which is useful for
    // referring to a part of an array or a string.
    let arr = [1, 2, 3];
    let slice = &arr[1..3]; // slice is a reference to a part of arr
    println!("slice: {:?}", slice);

    let s = String::from("hello, world!");
    let slice = &s[7..12]; // slice is a reference to a part of s
    println!("slice: {}", slice);

    // References can be used to create a mutable reference to a slice, which is useful
    // for modifying the value that the slice points to.
    let mut arr = [1, 2, 3];
    let slice = &mut arr[1..3]; // slice is a mutable reference to a part of arr
    slice[0] = 4;
    println!("arr: {:?}", arr);

    let mut s = String::from("hello, world!");
    let slice = &mut s[7..12]; // slice is a mutable reference to a part of s
    slice.push_str("!");
    println!("s: {}", s);

    // References can be used to create a reference to a struct, which is useful for
    // referring to a value that is stored in a struct.
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 1, y: 2 };
    let ref_p = &p; // ref_p is a reference to p
    println!("ref_p.x: {}", ref_p.x);
    println!("ref_p.y: {}", ref_p.y);

    // References can be used to create a mutable reference to a struct, which is useful
    // for modifying the value that the struct stores.
    let mut p = Point { x: 1, y: 2 };
    let mut ref_p = &mut p; // ref_p is a mutable reference to p
    ref_p.x = 3;
    ref_p.y = 4;
    println!("p.x: {}", p.x);
    println!("p.y: {}", p.y);

    // References can be used to create a reference to a enum, which is useful for
    // referring to a value that is stored in an enum.
    enum Color {
        Red,
        Green,
        Blue,
    }

    let c = Color::Green;
    let ref_c = &c; // ref_c is a reference to c
    println!("ref_c: {:?}", ref_c);

    // References can be used to create a mutable reference to an enum, which is useful
    // for modifying the value that the enum stores.
    let mut c = Color::Green;
    let mut ref_c = &mut c; // ref_c is a mutable reference to c
    *ref_c = Color::Blue;
    println!("c: {:?}", c);

    // References can be used to create a reference to a tuple, which is useful for
    // referring to a value that is stored in a tuple.
    let t = (1, 2);
    let ref_t = &t; // ref_t is a reference to t
    println!("ref_t.0: {}", ref_t.0);
    println!("ref_t.1: {}", ref_t.1);

    // References can be used to create a mutable reference to a tuple, which is useful
    // for modifying the value that the tuple stores.
    let mut t = (1, 2);
    let mut ref_t = &mut t; // ref_t is a mutable reference to t
    ref_t.0 = 3;
    ref_t.1 = 4;
    println!("t.0: {}", t.0);
    println!("t.1: {}", t.1);

    // References can be used to create a reference to an array, which is useful for
    // referring to a value that is stored in an array.
    let arr = [1, 2, 3];
    let ref_arr = &arr; // ref_arr is a reference to arr
    println!("ref_arr[0]: {}", ref_arr[0]);
    println!("ref_arr[1]: {}", ref_arr[1]);
    println!("ref_arr[2]: {}", ref_arr[2]);

    // References can be used to create a mutable reference to an array, which is useful
    // for modifying the value that the array stores.
    let mut arr = [1, 2, 3];
    let mut ref_arr = &mut arr; // ref_arr is a mutable reference to arr
    ref_arr[0] = 4;
    ref_arr[1] = 5;
    ref_arr[2] = 6;
    println!("arr[0]: {}", arr[0]);
    println!("arr[1]: {}", arr[1]);
    println!("arr[2]: {}", arr[2]);

    // References can be used to create a reference to a slice of an array, which is
    // useful for referring to a part of an array.
    let arr = [1, 2, 3];
    let slice = &arr[1..3]; // slice is a reference to a part of arr
    println!("slice[0]: {}", slice[0]);
    println!("slice[1]: {}", slice[1]);

    // References can be used to create a mutable reference to a slice of an array, which
    // is useful for modifying the value that the slice points to.
    let mut arr = [1, 2, 3];
    let mut slice = &mut arr[1..3]; // slice is a mutable reference to a part of arr
    slice[0] = 4;
    slice[1] = 5;
    println!("arr[1]: {}", arr[1]);
    println!("arr[2]: {}", arr[2]);
}
