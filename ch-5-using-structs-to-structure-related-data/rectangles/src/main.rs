// For using println macro with Rectangle
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Method
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated function
    // Can be used as constructors
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// Each struct can have different impl blocks, we can have all the functions inside separate impl blocks

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = (30, 50);

    println!(
        "The area of the rectangle using tuple params is {} square pixels.",
        area_with_tuples(rect1)
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle using struct params is {} square pixels.",
        area_with_struct(&rect1)
    );

    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);

    println!(
        "The area using struct's implementation is {} square pixels.",
        rect1.area()
    );

    let rect1 = Rectangle {
        width: 30, 
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10, 
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60, 
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // Calling associated functions
    let sq = Rectangle::square(3);
    println!("The area of square: {}", sq.area());
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_with_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_with_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}