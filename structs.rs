// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f64);

// A struct with two fields
struct Point {
    x: i32,
    y: i32,
}

// Structs can be used as fields for other structs
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // Create a unit struct
    let unit = Unit;

    // Create a tuple struct
    let pair = Pair(1, 0.1);

    // Create a struct with two fields
    let point: Point = Point { x: 10, y: 20 };

    // Create a struct with another struct as a field
    let rectangle = Rectangle {
        top_left: Point { x: 10, y: 20 },
        bottom_right: Point { x: 20, y: 30 },
    };

    // Access fields of a struct
    println!("point.x = {}", point.x);

    // We can also use pattern matching to access fields of a struct
    let Point { x: x1, y: y1 } = point;
    println!("x1 = {}, y1 = {}", x1, y1);

    // We can also use the dot notation to access fields of a struct
    println!("rectangle.top_left.x = {}", rectangle.top_left.x);
}
