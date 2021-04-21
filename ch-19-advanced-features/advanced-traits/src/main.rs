use std::ops::Add;
use std::fmt;

use fmt::write;

fn main() {
    //
    // Specifying placeholder types in trait definitions with associated types
    // 

    // Associated types connect a type placeholder with a trait
    // such that the trait method definitions can use these placeholder
    // types as their signatures

    // The implementor of the trait will specify the concrete type to be
    // used in this type's place for the particular implementation

    pub trait Iterator {
        type Item; // The associated type, stands in for the type of the values
                   // the type implementing the Iterator trait it iterating over

        fn next(&mut self) -> Option<Self::Item>;
    }

    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Counter {
            Counter {
                count: 0,
            }
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    // The difference between generics and associated types are that
    // when using generics we must annotate the types in each implementation

    // Even while using the next method on Counter, we would have to provide 
    // type annotations to indicate which implementation of Iterator we want to use

    //
    // Default generic type parameters and operator overloading
    //

    // If we specify a default concrete type then the implementors wouldn't have
    // to specify concrete type if the default type works

    // Operator overloading is a great use case of this technique

    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl std::ops::Add for Point {
        type Output = Point;
    
        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    // The default generic type is within Add trait here
    #[allow(unused)]
    trait Add<Rhs=Self> {
        type Output;

        fn add(self, rhs: Rhs) -> Self::Output;
    }

    struct Millimeters(u32);
    struct Meters(u32);

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }

    // Default type parameters are used in two ways

    // 1. To extend a type without breaking existing code

    // 2. To allow customization in specific cases most user's won't need

    // 
    // Fully qualified syntax for disambiguation: calling methods with the same name
    //

    // Rust doesn't prevent to have same method with same name in more than one trait

    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("*waving arms furiosly*");
        }
    }

    let person = Human;
    person.fly(); // Compiler will by default call the struct's own method

    // To use the trait's method we need to do this :-

    Pilot::fly(&person);
    Wizard::fly(&person);

    // Because fly method takes a self parameter, if we had two types
    // that both implement one trait, Rust could figure out which implementation to use

    // However, associated functions that are part of traits don't have a self parameter
    // When two types in the same scope implement the trait, Rust can't figure out which type
    // we mean unless we use a fully qualified syntax

    trait Animal {
        fn baby_name() -> String;
    }

    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }

    println!("A baby dog is called a {}", Dog::baby_name());

    // This will throw error as Animal::baby_name() is an associated function
    // rather than a method and thus doesn't have a self parameter
    // println!("A baby dog is called a {}", Animal::baby_name());

    // To disambiguate we will have to do this :-

    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    //
    // Using supertraits to require one trait's functionality within another trait
    //

    // Sometimes we might need one trait to use another trait's functionality 
    // In this case we need to rely on dependent trait also being implemented
    // The trait we rely on is a supertrait of the trait we are implementing

    // Use this trait only on those which already implement Display trait
    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string(); // Comes from Display
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    struct AnotherPoint {
        x: i32,
        y: i32
    }

    impl OutlinePrint for AnotherPoint {}

    impl fmt::Display for AnotherPoint {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    let another_pointer = AnotherPoint {
        x: 1,
        y: 3,
    };
    another_pointer.outline_print();

    // 
    // Using the newtype pattern to implement external traits on external types
    // 

    // The orphan rule states that we're allowed to implement a trait on a type
    // as long as either the trait or the type are local to our crate

    // It is possible to get around this restriction using the newtype pattern
    // which involves creating a new type in a tuple struct

    // The tuple struct will have one field and be a thin wrapper around the 
    // type we want to implement a trait for
    // Then the new wrapper type is local to our crate and we can implement 
    // the trait on the wrapper

    // There is no runtime performance penalty for using this pattern
    // and the wrapper is ommitted at compile time

    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", ")) // This uses self.0 to access the inner Vec<T>
        }
    }

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);

    // The downsize of using this is Wrapper is a new type and it won't have
    // the methods of the value it's holding
}
                