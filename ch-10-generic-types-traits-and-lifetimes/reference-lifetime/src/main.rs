use std::fmt::Display;

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Lifetime annotations in method definitions
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

fn main() {
    // The main aim of lifetimes is to prevent dangling references
    // which causes a program to reference data other than the data it's intended to reference

    // Rust checks for lifetime using something called as 'borrow checker'

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{:?}", i);

    // i (instance of struct) cannot outlive the reference it holds in its part field

    // Lifetimes on function or method parameters are called input lifetimes, and those for return value are
    // called output lifetimes

    // The lifetime ellision rules that rustc uses are:-

    // 1. If there is one parameter fn will have single lifetime parameter, for multiple parameters there will be 
    // distinct lifetime parameter for each - Applies for input lifetimes

    // 2. If there is a single parameter then that will be lifetime of return value - Applies for output lifetimes

    // 3. If there are values of type &self or &mut then output lifetime will have lifetime of &self - Applies
    // for output lifetimes

    // 'static lifetime means that the reference can live for the entire duration of the program
    // All string literals have 'static lifetime
    let _s: &'static str = "I have a static lifetime.";
    // The text of this string is stored directly in the compiled binary

    // Everything together
    fn _longest_with_an_announcement<'a, T>(
        x: &'a str,
        y: &'a str,
        ann: T,
    ) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}

// This won't work as the compiler won't be able to figure out the lifetime
// as it is dependent on the if else conditons
// To fix this we need to annotate the function with the generic lifetime parameter

// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// Function with lifetime annotations
// Here all will have the same lifetime 'a
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// This signature tells Rust that for some lifetime 'a, the function takes two parameters
// both of which are string slices that live atleast as long as lifetime 'a

// The function signature also tells Rust that the string slice returned fromt the function
// will live atleast as long as the lifetime 'a

// In practice, it means that the lifetime of the reference returned by the longest function
// is the same as the smaller of the lifetimes of the reference passed in

// This does not change the lifetime of any parameter or return values