#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 =30;
    let height1 = 50;

    println!("The area of the rectancle is {} square pixels.", area(width1, height1));

    let rect1 = (30, 50);

    println!("The area is {}", area2(rect1));

    // Refactored as a Struct
    let rect2 = Rectangle {
        width: 30, 
        height: 50
    };
    
    println!("rect2 is {:#?}", rect2);

    println!(
        "The Area of the rectangle is {} square pixels.", 
        area3(&rect2)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// Function sets the rectangle variable as a Rectangle struct and returns an integer value u32
fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
