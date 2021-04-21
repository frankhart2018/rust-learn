
fn main() {
    //
    // Using the newtype pattern for type safety and abstraction
    // 

    // Some other uses of newtype pattern are:-

    // 1. Statically enforcing the values so they are never confused 
    
    // 2. Indicating the units of a value

    // 3. Abstracting away some implementation details of a type
    // the new type can expose a public API that is different fromt he API
    // of the private inner type if we used the new type directly to restrict
    // the available functionality

    // 4. Can also hide internal implementation

    //
    // Creating type synonyms with type aliases
    // 

    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    // This is also commonly used in Result<T, E>

    type Result<T> = std::result::Result<T, std::io::Error>;

    // This can then be used as Result<T>

    //
    // The Never type that never returns  
    //

    fn bar() -> ! {
        panic!();
    }

    // This function will never return anything
    // Functions that return never are called diverging functions

    // This is helpful in match arms, as they can return a single type,
    // we can have integer for example in Ok arm and continue in Err arm
    // as continue has type ! so it never returns anything

    // If a match arm has to panic then also ! is useful

    // loop expressions can have !, example print!()

    // 
    // Dynamically sized types and sized trait
    //

    // str is a Dynamically Sized Type (DST), not &str but str
    // That's why we can't do this
    // let s1: str = "Hello, World!";

    // All dynamically sized types are basically used as references
    // which points to a memory location in heap
    // in case of &str it is the memory location and the length of string
    // this fixes the size of &str to 2 * usize

    // Every trait is a DST

    // To work with DSTs Rust has a particular trait called Sized trait
    // which determines whether or not a type's size is known at compile time

    // This trait is automatically implemented for everything whose
    // size is known at compile time

    // Rust also implicitly adds a bound on Sized to every generic function

    // So a function like
    // fn generic<T>(t: T)
    // is actually
    // fn generic<T: Sized>(t: T)

    // By default generic functions will work only on types that have a known size
    // at compile time
    // A special syntax to relax this restriction is
    // fn generic<T: ?Sized>(t: &T)
    // This syntax is available only for Sized trait and not other traits
    // Also in this case the type has to be switched from T to &T
    // Because if it is not Sized then we have to use it behind pointer
}
                