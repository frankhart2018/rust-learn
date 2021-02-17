#![allow(unused)]
fn main() {
    use std::fmt::Display;

    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        // This function can be called by only those instances of Pair
        // which have T as type that implements Display and PartialOrd traits
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }

    // Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations
    // E.g:- Standard library implements the ToString trait on any type that implements Display trait

    // This forces the errors of incompatible types in functions to compile time instead of runtime
}
