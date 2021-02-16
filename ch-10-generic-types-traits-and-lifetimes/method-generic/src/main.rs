struct Point<T> {
    x: T,
    y: T,
}

// By specifying T here we say that these methods inside impl can be used by an type of instance of Point
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// The functions inside this impl can be used only by Point instances of type f32
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct AnotherPoint<T, U> {
    x: T,
    y: U,
}

impl<T, U> AnotherPoint<T, U> {
    fn mixup<V, W>(self, other: AnotherPoint<V, W>) -> AnotherPoint<T, W> {
        AnotherPoint {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());

    let p1 = AnotherPoint { x: 5, y: 10.4 };
    let p2 = AnotherPoint { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

// Rust performs monomorphization - compile generics to concrete types, thus there is no runtime cost for using generics
