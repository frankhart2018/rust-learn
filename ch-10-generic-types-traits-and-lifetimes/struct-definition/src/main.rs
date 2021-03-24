#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct DiffPoint<T, U> {
    x: T,
    y: U,
}

fn main() {
    let integer = Point {
        x: 5, 
        y: 10,
    };

    let float = Point {
        x: 1.0,
        y: 4.0,
    };

    // Here both x and y should have the same type
    // If we assign different values it will throw error

    println!("{:?}", integer);
    println!("{:?}", float);

    // This has two generic parameters and can have different values
    let diff_point = DiffPoint {
        x: 4,
        y: 5.0
    };

    println!("{:?}", diff_point);
}
