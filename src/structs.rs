#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // Static method
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 100,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area(),
    );


    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.can_hold(&rect1)
    );
}
