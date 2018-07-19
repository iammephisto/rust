// Working with methods (functions within structs)

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// When you need to work with the same values in different ways, you can use a method
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn contour(&self) -> u32 {
        (self.width * 2)  + (self.height * 2)
    }

    fn canhold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {width: 30, height: 50};
    let rect2 = Rectangle {width: 20, height: 40};

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!(
        "The sides of the square total: {} pixels.",
        rect1.contour()
    );

    println!(
        "Can rect1 hold rect2? {}",
        rect1.canhold(&rect2)
    );
}
