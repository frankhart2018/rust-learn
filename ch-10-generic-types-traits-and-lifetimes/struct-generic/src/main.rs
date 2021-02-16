#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct AnotherPoint<T, U> {
    x: T,
    y: U,
}

fn main() {
    // Both x and y need to be of same type
    let integer = Point { x: 5, y: 10 };
    let float = Point{ x: 1.0, y: 4.0 };

    println!("{:?}", integer);
    println!("{:?}", float);

    // Both can have same or different types
    let both_integer = AnotherPoint { x: 5, y: 10 };
    let both_float = AnotherPoint { x: 1.0, y: 4.0 };
    let integer_and_float = AnotherPoint { x: 5, y: 4.0 };

    println!("{:?}", both_integer);
    println!("{:?}", both_float);
    println!("{:?}", integer_and_float);
}
