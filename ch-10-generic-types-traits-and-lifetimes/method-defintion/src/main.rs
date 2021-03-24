struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Implementations specific to i32 values only
impl Point<i32> {
    // Some functions
}

// 
// Different generic types in struct and methods
//
struct AnotherPoint<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p = Point {
        x: 5,
        y: 10,
    };

    println!("p.x = {}", p.x());
}
