
fn main() {
    //
    // Function pointers
    //

    // We can pass regular functions to functions using this method
    // Functions coerce to the type fn, this type is called function pointer

    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

    // Unlike closures fn is a type rather than a trait
    // So we can directly specify it as parameter

    // Function pointers implement all three of the closure traits (Fn, FnMut, FnOnce)
    // so we can always pass a function pointer as an argument for a function that expects a closure

    // It's best to write functions using a generic type and one of the closure 
    // traits so our functions can accept either functions or closures

    //
    // Returning closures
    //

    // Closures are represented by traits, which means we can't 
    // return closures directly, in most cases where we might want to return a trait
    // we can instead use the concrete type that implements the trait as the return 
    // value of the function
    // But this can't be done with closures because they don't have a concrete type
    // that is returnable

    // Example of how to return closure

    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }

    
}
                