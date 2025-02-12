struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Setter for width
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    // Setter for height
    fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    // Method to calculate area
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let mut rect = Rectangle { width: 10, height: 20 };

    println!("Initial area: {}", rect.area());

    // Use setters to modify dimensions
    rect.set_width(15);
    rect.set_height(25);

    println!("Updated area: {}", rect.area());
}

